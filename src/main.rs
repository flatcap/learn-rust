#![allow(unused)]
#![allow(unconditional_panic)]

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("The value of a[3] is: {}", a[3]);

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of b[3] is: {}", b[3]);

    let c: [i32; 5] = [1; 5]; // equivalent to [1, 1, 1, 1, 1]

    println!("The value of c[3] is: {}", c[3]);

    let _ = c[5];
}
