#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub const fn with_left(value: L) -> Either<L, R> {
        Either::Left(value)
    }

    pub const fn with_right(value: R) -> Either<L, R> {
        Either::Right(value)
    }

    pub const fn left(&self) -> Option<&L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    pub const fn left_mut(&mut self) -> Option<&mut L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    pub fn map_left<T, F: FnOnce(L) -> T>(self, f: F) -> Either<T, R> {
        match self {
            Either::Left(l) => Either::with_left(f(l)),
            Either::Right(r) => Either::with_right(r),
        }
    }

    pub fn take_left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    pub const fn right(&self) -> Option<&R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }

    pub const fn right_mut(&mut self) -> Option<&mut R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }

    pub fn map_right<T, F: FnOnce(R) -> T>(self, f: F) -> Either<L, T> {
        match self {
            Either::Left(l) => Either::with_left(l),
            Either::Right(r) => Either::with_right(f(r)),
        }
    }

    pub fn take_right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }
}
