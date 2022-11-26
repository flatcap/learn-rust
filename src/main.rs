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

fn iterator_map() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter().map(|x| x + 7);

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn iterator_filter() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );

    for shoe in in_my_size {
        println!("my size: {}: {}", shoe.size, shoe.style);
    }
}

fn main() {
    iterator_simple();
    iterator_sum();
    iterator_map();
    iterator_filter();
}
