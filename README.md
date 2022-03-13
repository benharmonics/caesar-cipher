# caesar_cipher
A simple Caesar Cipher encryption program

Usage: `caesar_cipher {offset} {path/text}`

The program takes two arguments, `offset`, which should be an integer in the range 0-26 inclusive, that will be used as the key which encrypts your text.
For example, if `offset` is 3 and your text is "abc", then the encrypted message will be "def". If `offset` is 0 or 26 and your text is "abc", the encrypted message will be "abc".

The `path/text` argument should be either the path to a text file, or a string to be encoded. If there is a space in your string, be sure to enclose the string in quotation
marks ("").
