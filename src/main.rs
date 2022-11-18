use std::io;
use std::io::Write;

fn main() {
    print!("Enter string: ");
    io::stdout().flush().unwrap();

    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");
    println!("");

    let sentence = sentence.trim();

    for word in sentence.split_whitespace() {
        print!("{} ", pig_latin(word));
    }
    println!("");
}

fn pig_latin(word: &str) -> String {
    if word.len() < 2 {
        return word.to_string();
    }

    let initial = &word.chars().next().unwrap().to_lowercase().next().unwrap();

    match initial {
        'a' | 'e' | 'i' | 'o' | 'u' => pig_vowel(word),
        'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' | 's'
        | 't' | 'v' | 'w' | 'x' | 'y' | 'z' => pig_consonant(word),
        _ => word.to_string(),
    }
}

fn pig_vowel(word: &str) -> String {
    format!("{}-hay", word)
}

fn pig_consonant(word: &str) -> String {
    if word.len() > 1 {
        format!("{}-{}ay", &word[1..], &word[..1])
    } else {
        word.to_string()
    }
}
