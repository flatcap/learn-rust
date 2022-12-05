use std::io;
use std::io::Write;

// Generate the n'th Fibonacci number
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 2) + fibonacci(n - 1),
    }
}

fn main() {
    loop {
        print!("Enter n: ");
        io::stdout().flush().unwrap();

        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n = n.trim();
        if n == "" {
            break;
        }

        let n: u32 = match n.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number");
                break;
            }
        };

        let fib = fibonacci(n);

        println!("Fibonacci number {n} is {fib}");
    }
}
