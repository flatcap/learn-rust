use std::io;
use std::io::Write;

fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    loop {
        print!("Enter number: ");
        io::stdout().flush().unwrap();

        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        let num = num.trim();
        if num == "" {
            break;
        }

        let num: i32 = match num.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                break;
            }
        };

        numbers.push(num);
    }

    let count = numbers.len();
    println!("Array contains {} numbers", count);

    numbers.sort();

    print!("{{ ");
    for i in &numbers {
        print!("{}, ", i);
    }
    println!("}}");

    let median = numbers.get(count / 2);
    match median {
        Some(median) => println!("The median is {}", median),
        None => println!("There is no median"),
    }
}
