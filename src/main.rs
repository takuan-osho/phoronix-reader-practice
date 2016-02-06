extern crate hyper;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name};
use select::node::Node;

struct Article {
    title: String,
    link: String,
    details: String,
    summary: String,
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
        let mut link = String::from(header.attr("href").unwrap());
        if link.starts_with("/") { assert_eq!(link.remove(0), '/'); }
        let mut details = node.find(Class("details")).first().unwrap().text();
        if details.contains("Add a comment") {
            details = details.replace("Add a Comment", "0 comments");
        }
        let summary = node.find(Name("p")).first().unwrap().text();
        Article{
            title: header.text(),
            link: link,
            details: details,
            summary: summary,
        }
    }
}

fn open_testing() -> &'static str {
    include_str!("phoronix.html")
}

fn main() {
    let phoronix_articles = Article::get_articles();
    for article in phoronix_articles {
        println!("Title:   {}", article.title);
        println!("Link:    https://www.phoronix.com/{}", article.link);
        println!("Details: {}", article.details);
        println!("Summary: {}\n", article.summary);
    }
}