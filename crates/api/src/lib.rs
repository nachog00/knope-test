pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// API endpoint that returns a greeting using core's hello function
pub fn greet_endpoint() -> String {
    let greeting = myapp_core::hello();
    format!("API Response: {}", greeting)
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
