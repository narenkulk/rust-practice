pub mod aggregator; // specifying pub here will expose the aggregator mod so that main.rs can use it.
use aggregator::tweet;
pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for crate::aggregator::newsarticle::NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for tweet::Tweet {
    fn summarize(&self) -> String {
        format!("{} by {}", self.content, self.username)
    }
}
