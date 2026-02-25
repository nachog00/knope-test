use std::env;

/// CLI argument configuration (breaking: changed from positional to named args)
struct Config {
    verbose: bool,
    output_format: String,
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    Config {
        verbose: args.contains(&"--verbose".to_string()),
        output_format: args
            .iter()
            .position(|a| a == "--format")
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| "text".to_string()),
    }
}

fn main() {
    let config = parse_args();
    if config.verbose {
        println!("Output format: {}", config.output_format);
    }
    println!("Hello, world!");
}
