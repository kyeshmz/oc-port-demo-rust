const BASE_DELAY_MS: u64 = 250;
const MAX_DELAY_MS: u64 = 2000;

/// Exponential retry delay with a cap, for a zero-based unsigned attempt index.
pub fn retry_delay_ms(attempt: u32) -> u64 {
    BASE_DELAY_MS
        .saturating_mul(2_u64.saturating_pow(attempt))
        .min(MAX_DELAY_MS)
}

#[cfg(test)]
mod tests {
    use super::retry_delay_ms;

    #[test]
    fn grows_exponentially_and_caps() {
        let actual: Vec<u64> = (0..=4).map(retry_delay_ms).collect();
        assert_eq!(actual, vec![250, 500, 1000, 2000, 2000]);
    }

    #[test]
    fn huge_attempts_do_not_overflow() {
        assert_eq!(retry_delay_ms(32), 2000);
        assert_eq!(retry_delay_ms(u32::MAX), 2000);
    }
}
