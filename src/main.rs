#![allow(unused)]

use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
