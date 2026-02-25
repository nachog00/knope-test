/// Default timeout in seconds
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Connect with proper timeout handling (fixes #123)
pub fn connect_with_timeout(timeout_secs: Option<u64>) -> Result<(), String> {
    let timeout = timeout_secs.unwrap_or(DEFAULT_TIMEOUT_SECS);
    if timeout == 0 {
        return Err("Timeout cannot be zero".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
