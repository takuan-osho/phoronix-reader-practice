use std::io::Read;

extern crate hyper;
use hyper::Client;
use hyper::header::Connection;

extern crate select;
mod article;
use article::Article;

fn open_phoronix() -> String {
    let client = Client::new();
    let mut response = client.get("https://www.phoronix.com/")
        .header(Connection::close()).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    return body;
}

fn main() {
    let phoronix_articles = Article::get_articles(&open_phoronix());
    for article in phoronix_articles.iter().rev() {
        println!("Title:   {}", article.title);
        println!("Link:    https://www.phoronix.com/{}", article.link);
        println!("Details: {}", article.details);
        println!("Summary: {}\n", article.summary);
    }
}