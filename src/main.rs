use std::io;
use std::io::Write;

fn main() {
    print!("Enter string: ");
    io::stdout().flush().unwrap();
    println!("");

    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");

    let sentence = sentence.trim();

    for word in sentence.split_whitespace() {
        print!("{} ", pig_latin(word));
    }
    println!("");
}

fn pig_latin(word: &str) -> String {
    match word.chars().next() {
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => { "vowel".to_string() }
        _ => { "consonant".to_string() }
    }
}
