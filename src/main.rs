#![allow(unused)]

use crate::List::{Cons, Nil};
use std::ops::Deref;

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

fn test_deref() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn test_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn test_double() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn main() {
    test_box();
    test_list();
    test_deref();
    test_mybox();
}
