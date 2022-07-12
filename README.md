# caesar-cipher
A simple Caesar Cipher encryption program

Usage: `caesar-cipher [offset] <path/text>`

The program takes two arguments, `offset`, which should be a non-negative integer less than 256, that will be used as the key which encrypts your text.
For example, if `offset` is 3 and your text is "abc", then the encrypted message will be "def". If `offset` is 0 or 26 and your text is "abc", the encrypted message will be "abc".

Typically, `offset` should be between 0 and 26. You can use a higher offset but if it can't be parsed, it'll just default to 13. If you don't enter an offset it'll also default to 13.

The `path/text` argument should be either the path to a text file, or a string to be encoded. If there is a space in your string, be sure to enclose the string in quotationmarks, i.e. `caesar_cipher 'spaces in this text'`.

Note that non-alphabetic characters will not be updated and the string will not be reformatted, other than 'rotating' the letters.

## Installation

### Cargo
Cargo comes with Rust.

Clone the repository into a local directory, then:
```
cargo install --path path/to/caesar-cipher
```
