use traits::{Tweet, Summary, notify, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "Hello, fellow readers".to_string(),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let news_article = NewsArticle {
        author: "Jonathan Bluett-Duncan".to_string(),
        content: "The sqrt of pi is an irrational number".to_string(),
        location: "London, UK".to_string(),
        headline: "Breaking Maths News".to_string(),
    };
    notify(&tweet);
    notify(&news_article);
}
