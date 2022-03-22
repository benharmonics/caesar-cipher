pub struct Config {
    offset: u8,
    content: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        // offset: the amount each letter will be 'rotated' to the right using the caesar cipher function.
        // i.e. an offset of 3 would take the content "abc" and output "def".
        // Note that to decode a message that has been encoded with offset n, use a decoder offset of 26 - n.
        let offset = match args.next() {
            Some(arg) => match arg.parse::<u8>() {
                Ok(n) => n % 26,    // Technically an offset of 27 should be equivalent to an offset of 1, for instance.
                Err(_) => return Err("Offset must be a non-negative integer, typically from 1-25."),
            }
            None => return Err("Usage: caesar_cipher {offset} {filename/string}"),
        };
        // content: the content to be encoded by the caesar cipher. See mod tests below for examples.
        let content: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Usage: caesar_cipher {offset} {filename/string}"),
        };

        if let Some(_) = args.next() {
            println!("You may have meant to enclose a string in quotation marks. caesar_cipher takes exactly two arguments.");
            println!("Example: caesar_cipher 13 \"multiple words\"");
        }

        Ok(Config { offset, content })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Try to use config.content as a path to a text file; if that fails, treat it as a string in and of itself.
    if let Ok(content) = std::fs::read_to_string(&config.content) {
        println!("{}", caesar_cipher(&content, config.offset));
    } else {
        println!("{}", caesar_cipher(&config.content, config.offset));
    }

    Ok(())
}

// Caesar Cipher rotates all letters by a fixed number n. You might know this as a RotN cipher?
// A common example would be a Rot13 cipher. See mod tests below for examples.
fn caesar_cipher(text: &str, shift: u8) -> String {
    text
        .chars()
        .map(|c| {
            match c.is_ascii_alphabetic() {
                true => {
                    let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    (first + (c as u8 + shift - first) % 26) as char
                }
                false => c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::caesar_cipher;

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
        assert_eq!(String::from("jU xPsLt XjUi NjYfE dBtFt"), caesar_cipher(cipher, 1));
    }
} 
