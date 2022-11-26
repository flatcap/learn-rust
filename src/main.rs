fn iterator_simple() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // sum() consumes the iterator
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
    println!("total = {total}");
}

fn main() {
    iterator_simple();
    iterator_sum();
}
