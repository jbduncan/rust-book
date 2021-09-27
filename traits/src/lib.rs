use std::fmt::{Display, Debug};

pub trait Summary {
    // 1. Define a method with no body
    // fn summarize(&self) -> String;

    // 2. Define a method with a default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("By {}", self.author)
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

// ------------------

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// Equivalent to:
fn notify2<T: Summary>(item: &T) {
    unimplemented!()
}

// We can also combine trait bounds:
fn notify3(item: &(impl Summary + Display)) {
    unimplemented!()
}

fn notify4<T: Summary + Display>(item: &T) {
    unimplemented!()
}

// We can use `where` rather than `<>` to specify generics if we wish:
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {
    unimplemented!()
}

// Equivalent to:
fn some_function_2<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    unimplemented!()
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "horse_ebooks".to_string(),
        content: "Hello, fellow readers".to_string(),
        reply: false,
        retweet: false,
    }
}

// Above function doesn't work if it returns either a Tweet or NewsArticle...
