#[inline(always)]
pub fn atoi_u64(str: &str) -> u64 {
    str.bytes().fold(0u64, |acc, v| 10 * acc + (v - b'0') as u64)
}