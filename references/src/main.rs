fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{s}");
}

fn change(some_string: &mut String) {
    change_deeper(some_string);
}

fn change_deeper(some_string: &mut String) {
    change_deeper_still(some_string);
}

fn change_deeper_still(some_string: &mut String) {
    some_string.push_str(", world");
}
