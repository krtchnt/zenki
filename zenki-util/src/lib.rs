#[inline]
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub const fn usize_to_i32(n: usize) -> i32 {
    n as i32
}

#[inline]
#[allow(clippy::cast_sign_loss)]
pub const fn i32_to_usize(n: i32) -> usize {
    n as usize
}
