// consider a media library that needs to provide a way to summarise many types of article
// a trait defines all the behaviour each article is guaranteed to have implemented
pub trait Summarise {
    // define the name, and function signature but no behaviour
    fn summarise(&self) -> String;

    // default implementation - fill in the default behaviour that is overwritten when needed
    fn message(&self) {
        println!("Default Message");
    }
}

pub trait Display {
    fn display(&self) -> String;
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

impl Display for NewsArticle {
    fn display(&self) -> String {
        format!("{} \n {}, \n {} ({})", self.headline, self.content, self.author, self.location)
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
    fn message(&self) {
        println!("This is a Social Article!");
    }
}


// using the implementation &impl syntax to specify a type signature for a function argument
// this means that any type which implements the Summarise trait is valid
pub fn notify(item: &impl Summarise) {
    println!("Breaking news!! {}", item.summarise());
}

// it turns out that the &impl Trait syntax is syntactic sugar
// this is the full detail (using Trait Bounds)

pub fn notify_longer<T: Summarise>(item: &T) {
    println!("Breaking news!! {}", item.summarise());
}

// if we need one type to implement two or more traits: use the + syntax
pub fn show_all<T: Summarise + Display>(item: &T) {
    println!("Summary:");
    println!("{}", item.summarise());
    println!("Full article:");
    println!("{}", item.display());
}


// using where clauses
// to make function signatures less vervose, more maintainable and easier to understand
pub fn some_fun<T, U>(t: &T, u: &U)
where
    T: Display + Summarise + Clone,
    U: Summarise + Copy,
    {
        println!("complicated!!!");
    }

// functions can also return any type that implements a certain trait:
pub fn returns_summarisable() -> impl Summarise {
    SocialArticle {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}


fn main() {
    let news = NewsArticle {
        headline: String::from("Programmer does Rust"),
        location: String::from("London"),
        author: String::from("Reuben Stannah"),
        content: String::from("In an amazing turn of events, a local programmer has started to learn rust"),
    };

    let social = SocialArticle {
        username: String::from("reubenstannah"),
        content: String::from("Hi guys! doing more coding"),
        reply: false,
        repost: false,
    };

    println!("{}", social.summarise());
    println!("{}", news.summarise());

    // using the default implementation for Summarise
    news.message();

    // using the newly defined message (overwriting the default version)
    social.message();

    // using the notify function
    notify(&news);
    notify(&social);

    println!("Now, without using syntactic sugar:");

    // without syntactic sugar
    notify_longer(&news);
    notify_longer(&social);

    // using a function for something which must implement two (or more) traits
    show_all(&news);

    let summarisable = returns_summarisable();
    println!("{}", summarisable.summarise());

}
