#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{c_for, c_for_rev};

#[macro_export]
macro_rules! bit_array_with_min_len {
    (0) => {
        $crate::bitset::BitArray::<0>::new()
    };
    ($nbits:literal) => {
        $crate::bitset::BitArray::<{ ($nbits - 1) / (8 * size_of::<usize>()) + 1usize }>::new()
    };
}

macro_rules! chunk_size {
    ($u:ty) => {
        (8 * size_of::<$u>())
    };
}

macro_rules! overflow_bit_mask {
    ($u:ty) => {
        (<$u>::MAX << (chunk_size!($u) - 1))
    };
}

macro_rules! underflow_bit_mask {
    ($u:ty) => {
        (1 as $u)
    };
}

macro_rules! lhs_overflow_bit {
    ($chunk:expr, $u:ty) => {
        ((($chunk) & overflow_bit_mask!($u)) >> (chunk_size!($u) - 1))
    };
}

macro_rules! rhs_underflow_bit {
    ($chunk:expr, $u:ty) => {
        ((($chunk) & underflow_bit_mask!($u)) << (chunk_size!($u) - 1))
    };
}

macro_rules! impl_shared_fns {
    ($u:ty $(,$uconst:tt)?) => {
        #[inline(always)]
        pub const fn chunk_count(&self) -> usize {
            self.chunks.len()
        }

        #[inline(always)]
        pub const fn len(&self) -> usize {
            self.chunk_count() * chunk_size!($u)
        }

        #[inline(always)]
        pub $($uconst)? fn at(&self, bit: usize) -> bool {
            *self.at_ref(bit)
        }

        #[inline(always)]
        pub $($uconst)? fn is_empty(&self) -> bool {
            let end = self.chunk_count();
            c_for!(i in 0..end {
               if self.chunks[i] > 0 {
                   return false;
               }
            });
            true
        }

        #[inline(always)]
        pub $($uconst)? fn set(&mut self, bit: usize) {
            debug_assert!(bit < self.len());
            let (chunk, bit) = (bit / chunk_size!($u), bit % chunk_size!($u));
            self.chunks[chunk] |= (1 as $u) << bit;

        }

        #[inline(always)]
        pub $($uconst)? fn nul(&mut self, bit: usize) {
            debug_assert!(bit < self.len());
            let (chunk, bit) = (bit / chunk_size!($u), bit % chunk_size!($u));
            self.chunks[chunk] &= !((1 as $u) << bit);
        }

        #[inline(always)]
        /// inserts a `1` on bit offset `bit`
        pub $($uconst)? fn insert(&mut self, bit: usize) {
            debug_assert!(bit < self.len());
            let (chunk, bit) = (bit / chunk_size!($u), bit % chunk_size!($u));
            let mut overflow = lhs_overflow_bit!(self.chunks[chunk], $u);
            let tmp = self.chunks[chunk] & !(<$u>::MAX << bit);
            self.chunks[chunk] <<= 1;
            self.chunks[chunk] &= <$u>::MAX << (bit + 1);
            self.chunks[chunk] |= tmp;
            self.chunks[chunk] |= (1 as $u) << bit;

            let start = chunk+1;
            let end = self.chunk_count();
            c_for!(sh_chunk in start..end {
                let new_overflow = lhs_overflow_bit!(self.chunks[sh_chunk], $u);
                self.chunks[sh_chunk] <<= 1;
                self.chunks[sh_chunk] |= overflow;
                overflow = new_overflow;
            });
        }

        #[inline(always)]
        /// removes the bit on bit offset `bit`
        pub $($uconst)? fn erase(&mut self, bit: usize) {
            debug_assert!(bit < self.len());
            let (chunk, bit) = (bit / chunk_size!($u), bit % chunk_size!($u));
            let mut underflow = 0;
            let start = chunk + 1;
            let end = self.chunk_count();
            c_for_rev!(sh_chunk in start..end {
                let new_underflow = rhs_underflow_bit!(self.chunks[sh_chunk], $u);
                self.chunks[sh_chunk] >>= 1;
                self.chunks[sh_chunk] |= underflow;
                underflow = new_underflow;
            });

            let tmp = self.chunks[chunk] & !(<$u>::MAX << bit);
            self.chunks[chunk] >>= 1;
            self.chunks[chunk] &= <$u>::MAX << bit;
            self.chunks[chunk] |= tmp;
            self.chunks[chunk] |= underflow;
        }

        #[inline(always)]
        /// Swaps bit `bit1` and bit `bit2`
        pub $($uconst)? fn swap(&mut self, bit1: usize, bit2: usize) {
            debug_assert!(bit1 < self.len());
            debug_assert!(bit2 < self.len());
            let (chunk1, bit1) = (bit1 / chunk_size!($u), bit1 % chunk_size!($u));
            let (chunk2, bit2) = (bit2 / chunk_size!($u), bit2 % chunk_size!($u));

            let mask1 = ((1 as $u) << bit1);
            let mask2 = ((1 as $u) << bit2);

            let val1 = self.chunks[chunk1] & mask1;
            let val2 = self.chunks[chunk2] & mask2;

            let moved1 = (val1 >> bit1) << bit2;
            let moved2 = (val2 >> bit2) << bit1;

            self.chunks[chunk1] &= !mask1;
            self.chunks[chunk2] &= !mask2;

            self.chunks[chunk1] |= moved2;
            self.chunks[chunk2] |= moved1;
        }

        $($uconst)? fn at_ref(&self, bit: usize) -> &'static bool {
            debug_assert!(bit < self.len());
            let (chunk, bit) = (bit / chunk_size!($u), bit % chunk_size!($u));
            if (self.chunks[chunk] >> bit) & 1 != 0 {
                &true
            } else {
                &false
            }
        }
    };
}

macro_rules! impl_bitset {
    ($u:ty) => {
        impl<const N: usize> BitSet<[$u; N]> {
            pub const fn new() -> BitSet<[$u; N]> {
                BitSet { chunks: [0; N] }
            }

            pub const fn from_array(array: [$u; N]) -> BitSet<[$u; N]> {
                BitSet { chunks: array }
            }
        }

        impl<const N: usize> BitSet<[$u; N]> {
            impl_shared_fns!($u, const);
        }

        impl<const N: usize> std::ops::Index<usize> for BitSet<[$u; N]> {
            type Output = bool;

            fn index(&self, index: usize) -> &Self::Output {
                debug_assert!(index < self.len());
                let (chunk, bit) = (index / chunk_size!($u), index % chunk_size!($u));
                if (self.chunks[chunk] >> bit) & 1 != 0 {
                    &true
                } else {
                    &false
                }
            }
        }

        impl<const N: usize> From<[$u; N]> for BitSet<[$u; N]> {
            fn from(value: [$u; N]) -> BitSet<[$u; N]> {
                BitSet::<[$u; N]>::from_array(value)
            }
        }

        impl BitSet<Vec<$u>> {
            pub fn new(min_len: usize) -> BitSet<Vec<$u>> {
                if min_len == 0 {
                    BitSet { chunks: vec![] }
                } else {
                    BitSet {
                        chunks: vec![0; (min_len - 1) / (8 * size_of::<usize>()) + 1usize],
                    }
                }
            }

            pub const fn from_vec(vec: Vec<$u>) -> BitSet<Vec<$u>> {
                BitSet { chunks: vec }
            }

            pub fn resize(&mut self, min_len: usize) {
                if min_len == 0 {
                    self.chunks.clear();
                } else {
                    self.chunks
                        .resize((min_len - 1) / (8 * size_of::<usize>()) + 1usize, 0);
                }
            }
        }

        impl BitSet<Vec<$u>> {
            impl_shared_fns!($u);
        }

        impl std::ops::Index<usize> for BitSet<Vec<$u>> {
            type Output = bool;

            fn index(&self, index: usize) -> &Self::Output {
                debug_assert!(index < self.len());
                let (chunk, bit) = (index / chunk_size!($u), index % chunk_size!($u));
                if (self.chunks[chunk] >> bit) & 1 != 0 {
                    &true
                } else {
                    &false
                }
            }
        }

        impl From<Vec<$u>> for BitSet<Vec<$u>> {
            fn from(value: Vec<$u>) -> BitSet<Vec<$u>> {
                BitSet::<Vec<$u>>::from_vec(value)
            }
        }
    };
}

pub type BitArray<const N: usize> = BitSet<[usize; N]>;
pub type BitVec = BitSet<Vec<usize>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BitSet<Storage = Vec<usize>> {
    chunks: Storage,
}

impl_bitset!(u8);
impl_bitset!(u16);
impl_bitset!(u32);
impl_bitset!(u64);
impl_bitset!(u128);
impl_bitset!(usize);
