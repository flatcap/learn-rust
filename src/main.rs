fn gift(day: u32) {
    const GIFTS: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for index in (0..day).rev() {
        let index = index as usize;
        println!("\t{}", GIFTS[index]);
    }
}

fn sing(day: u32) {
    const ORDINALS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelveth",
    ];

    let index = (day - 1) as usize;
    println!(
        "On the {} day of Christmas my true love sent to me:",
        ORDINALS[index]
    );

    gift(day);
    println!("");
}

fn main() {
    for day in 1..=12 {
        sing(day);
    }
}
