use caesar_cipher::{run, Config};
use std::env;

fn main() {
    run(Config::from(env::args()))
}
