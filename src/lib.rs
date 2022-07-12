use std::env::Args;
use std::{fs, process};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn run(config: Config) {
    // Try to use config.content as a path to a text file; if that fails, treat it as a string in and of itself.
    match fs::read_to_string(&config.content) {
        Ok(content) => print!("{}", caesar_cipher(&content, config.offset)),
        Err(_) => println!("{}", caesar_cipher(&config.content, config.offset)),
    }
}

pub struct Config {
    offset: u8,
    content: String,
}

impl Config {
    pub fn from(args: Args) -> Config {
        let args: Vec<_> = args.collect();
        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            print!("\x1b[1;96;127m{}\x1b[0m", PKG_NAME);
            println!(" - Encodes a string using a Caesar cipher");
            println!("    author: benharmonics");
            println!("    https://github.com/benharmonics/caesar_cipher");
            println!("\x1b[1;96;127m\nUSAGE:\x1b[0m");
            println!("  Encode a file:");
            println!("    {} <offset> <filename>", PKG_NAME);
            println!("  Encode a string:");
            println!("    {} <offset> <string>", PKG_NAME);
            println!("  Enclose a string in quotes if it has spaces in it, i.e.:");
            println!("    {} 'spaces in this string'", PKG_NAME);
            println!("\x1b[1;96;127m\nFLAGS:\x1b[0m");
            println!("  -d --decode   Decode a message with a known rotation");
            println!("  -h --help     Show help");
            println!("\x1b[1;96;127m\nEXAMPLES:\x1b[0m");
            println!("    {} 3 abc", PKG_NAME);
            println!("  output: def");
            println!("    {} -d 3 def", PKG_NAME);
            println!("  output: abc");
            process::exit(1);
        }
        // offset: the amount each letter will be 'rotated' to the right using the caesar cipher function.
        // i.e. an offset of 3 would take the content "abc" and output "def".
        // Note that to decode a message that has been encoded with offset n, use a decoder offset of 26 - n.
        let offset_input = args.iter().find_map(|s| s.parse::<u8>().ok()).unwrap_or(13);
        let offset = if args.contains(&"-d".to_string()) || args.contains(&"--decode".to_string()) {
            26 - (offset_input % 26)
        } else {
            offset_input
        };

        // content: the content to be encoded by the caesar cipher. See mod tests below for examples.
        let content = args[1..]
            .iter()
            .find_map(|s| {
                (s.parse::<u8>().is_err() && !s.starts_with('-')).then(|| String::from(s))
            })
            .unwrap_or_default();

        Config { offset, content }
    }
}

// Caesar Cipher rotates all letters by a fixed number n. You might know this as a RotN cipher?
// A common example would be a Rot13 cipher. See mod tests below for examples.
fn caesar_cipher(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| match c.is_ascii_alphabetic() {
            true => {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (first + (c as u8 + shift - first) % 26) as char
            }
            false => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(String::from("uryyb"), caesar_cipher("hello", 13));
    }

    #[test]
    fn it_works_with_punctuation() {
        assert_eq!(String::from("uryyb! |~."), caesar_cipher("hello! |~.", 13));
    }

    #[test]
    fn it_works_with_mixed_cases() {
        let cipher = "iT wOrKs WiTh MiXeD cAsEs";
        assert_eq!(
            String::from("jU xPsLt XjUi NjYfE dBtFt"),
            caesar_cipher(cipher, 1)
        );
    }
}
