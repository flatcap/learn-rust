use std::collections::HashMap;
use std::io;

fn main() {
    let mut depts: HashMap<String, String> = HashMap::new();

    loop {
        // println!("Enter fruit and subject: ");

        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");
        if text.len() == 0 {
            break;
        }

        let mut iter = text.split_whitespace();
        let fruit = iter.next();
        let dept = iter.next();

        if !fruit.is_some() || !dept.is_some() {
            println!("Too few fields");
            break;
        }

        if iter.next().is_some()
        {
            println!("Too many fields");
            break;
        }

        let fruit = fruit.unwrap().to_string();
        let dept = dept.unwrap().to_string();

        // println!("{} -> {}", fruit, dept);

        depts.insert(fruit, dept);
    }

    for (fruit, dept) in depts {
        println!("{} -> {}", fruit, dept);
    }
}
