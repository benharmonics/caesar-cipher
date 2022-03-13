pub struct Config {
    pub offset: u32,
    pub content: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let offset = match args.next() {
            Some(arg) => arg.parse().unwrap_or(200),    // causes Err below (line 20); kind of hack-y...
            None => return Err("Usage: caesar_cipher {offset} {filename/string}"),
        };

        let content = match args.next() {
            Some(arg) => arg,
            None => return Err("Usage: caesar_cipher {offset} {filename/string}"),
        };

        if offset > 26 { return Err("Offset must be an integer in the range 0-26.") }

        Ok(Config { offset, content })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {

    if let Ok(contents) = std::fs::read_to_string(&config.content) {
        println!("{}", caesar_cipher(&contents, config.offset));
    } else {
        println!("{}", caesar_cipher(&config.content, config.offset));
    }

    Ok(())
}

// Caesar Cipher rotates all letters by a fixed number.
// A common example would be a Rot13 cipher.
fn caesar_cipher(contents: &str, n: u32) -> String {
    contents
        .chars()
        .map(|c| c.to_ascii_lowercase() as u32 + n)     // increment ascii values by n
        .map(|d| if d > 122 { d - 26 } else { d })  // ASCII 'z' = 122
        .map(|d| if 97 <= d && d <= 122 { d } else { d - n })   // revert punctuation & spaces
        .map(|d| char::from_u32(d).unwrap_or('?'))
        .collect::<String>()
}