// a trait can have multiple method in its body.
// method signatures are listed one per line and each line ends in a semicolon
pub trait Summary {
    fn summarize(&self) -> String;
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
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let news: NewsArticle = NewsArticle {
        headline: "Rust Rules!".to_string(),
        location: "Kyoto, Japan".to_string(),
        author: "Mutyu".to_string(),
        content: "We've got rules!".to_string(),
    };

    let social_post: SocialPost = SocialPost {
        username: "@mutyu".to_string(),
        content: "I love Rust!".to_string(),
        reply: true,
        repost: false,
    };

    println!("{}", news.summarize());
    println!("{}", social_post.summarize());
}