
#[inline(always)]
pub const fn ceil_div(num: usize, div: usize) -> usize {
    debug_assert!(div > 0);
    (num + div - 1) / div
}
