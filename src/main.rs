extern crate hyper;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name};
use select::node::Node;

struct Article {
    title: String,
    // link: String,
    // details: String,
    // summary: String,
}

impl Article {
    fn get_articles() -> Vec<Article> {
        Document::from_str(open_testing())
            .find(Name("article")).iter()
            .map(|node| Article::new(&node))
            .collect()
    }
    
    fn new(node: &Node) -> Article {
        let header = node.find(Name("a")).first().unwrap();
        Article{ title: header.text() }
    }
}

fn open_testing() -> &'static str {
    include_str!("phoronix.html")
}

fn main() {
    let phoronix_articles = Article::get_articles();
    for article in phoronix_articles {
        println!("{}", article.title);
    }
}