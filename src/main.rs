use std::io;

// Convert degrees Fahrenheit to Celsius
fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Convert degrees Celsius to Fahrenheit
fn c_to_f(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn main() {
    println!("Enter temperature:");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => todo!(),
    };

    println!("{:.2}℃ is {:.2}℉", temp, c_to_f(temp));
    println!("{:.2}℉ is {:.2}℃", temp, f_to_c(temp));
}
