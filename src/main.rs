use caesar_cipher::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });
    
    if let Err(e) = caesar_cipher::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}