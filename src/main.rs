#![allow(unused)]

use std::fmt;
use std::io::Error;

fn type_longhand() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        //
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        Box::new(|| ())
    }
}

fn type_aliased() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        //
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| ())
    }
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

type Result2<T> = std::result::Result<T, std::io::Error>;

pub trait Write2 {
    fn write(&mut self, buf: &[u8]) -> Result2<usize>;
    fn flush(&mut self) -> Result2<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result2<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result2<()>;
}

fn main() {}
