#![allow(unused)]

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn test_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn test_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn main() {
    test_box();
    test_list();
}
