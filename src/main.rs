
fn main() {
    let config = caesar_cipher::Config::new(std::env::args())
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
    
    if let Err(e) = caesar_cipher::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
