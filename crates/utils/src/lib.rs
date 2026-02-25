pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Formats a message with a prefix and timestamp placeholder
pub fn format_message(prefix: &str, message: &str) -> String {
    format!("[{}] {}", prefix, message)
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
