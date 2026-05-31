// consider a media library that needs to provide a way to summarise many types of article
// a trait defines all the behaviour each article is guaranteed to have implemented
pub trait Summarise {
    // define the name, and function signature but no behaviour
    fn summarise(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implementing a trait for the NewsArticle struct
impl Summarise for NewsArticle {
    fn summarise(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

// now implementing the same method for a different struct
pub struct SocialArticle {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summarise for SocialArticle {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


fn main() {
    println!("Hello, world!");
}
