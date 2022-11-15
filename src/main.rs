#![allow(unused)]

fn main() {
    let y = {
        let x = 3;
        x + 1  // note: no semicolon
    };

    println!("The value of y is: {y}");
}
