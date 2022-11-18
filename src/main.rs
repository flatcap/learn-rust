#![allow(unused)]

use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
