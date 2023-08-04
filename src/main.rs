use std::env;
use std::process;
use minigrep::Config;

/// run:
/// IGNORE_CASE=1 cargo run 
/// to run minigrep on insensitive case mode
/// cargo run without the env var will result 
/// in case sensitive search
fn main() {
    let args: Vec<String> = env::args().collect();
    // |err| is a closure (anonymous function)
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("{e}");
        process::exit(1);
    }
}

