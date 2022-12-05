#![allow(unused)]

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("'{}'", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// This function is shorthand for the one below
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/* Trait bound syntax:
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// This form allows item1 and item2 to have different types, as long as they both implement Summary
pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// This form forces item1 and item2 to be exactly the same type, that implements Summary
pub fn notify<T: Summary>(item1: &T, item2: &T) {

// Require multiple traits with the '+' syntax (short form)
pub fn notify(item: &(impl Summary + Display)) {

// Require multiple traits with the '+' syntax (long form)
pub fn notify<T: Summary + Display>(item: &T) {


// SO MANY traits
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// Abbreviated using 'where' clauses:
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
*/

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
