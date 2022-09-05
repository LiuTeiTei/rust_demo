use trait_demo::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    };
    println!("tweet: {}", tweet.summarize()); // tweet: username: content

    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };
    println!("article {}", article.summarize()); // article headline, by author (location)
    println!("article {}", NewsArticle::summarize(article)); // article author: content
}
