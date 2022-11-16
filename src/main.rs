// Convert degrees Fahrenheit to Celsius
fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Convert degrees Celsius to Fahrenheit
fn c_to_f(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn main() {
    let temp_c: [f64; 5] = [-40.0, -20.0, 0.0, 36.9, 100.0];
    let temp_f: [f64; 5] = [-40.0, 0.0, 32.0, 98.4, 212.0];

    for t in temp_c {
        println!("{}℃ is {}℉", t, c_to_f(t));
    }

    for t in temp_f {
        println!("{}℉ is {}℃", t, f_to_c(t));
    }
}
