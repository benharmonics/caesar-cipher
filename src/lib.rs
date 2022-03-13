pub struct Config {
    offset: u32,
    content: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let offset: u32 = match args.next() {
            Some(arg) => match arg.parse::<u32>() {
                Ok(n) => n % 26,
                Err(_) => return Err("Offset must be a non-negative integer, typically from 1-25."),
            }
            None => return Err("Usage: caesar_cipher {offset} {filename/string}"),
        };

        let content: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Usage: caesar_cipher {offset} {filename/string}"),
        };

        Ok(Config { offset, content })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(content) = std::fs::read_to_string(&config.content) {
        println!("{}", caesar_cipher(&content, config.offset));
    } else {
        println!("{}", caesar_cipher(&config.content, config.offset));
    }

    Ok(())
}

// Caesar Cipher rotates all letters by a fixed number.
// A common example would be a Rot13 cipher.
fn caesar_cipher(content: &str, n: u32) -> String {
    content
        .chars()
        .map(|c| c.to_ascii_lowercase()) 
        .map(|c| if c.is_ascii_alphabetic() { 
            let d = c as u32 + n;
            if d > 122 { d - 26 } else { d }
        } else {
            c as u32
        })
        .map(|d| char::from_u32(d).unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(String::from("uryyb"), super::caesar_cipher("hello", 13));
    }

    #[test]
    fn it_works_with_punctuation() {
        assert_eq!(String::from("uryyb! |~."), super::caesar_cipher("hello! |~.", 13));
    }
}