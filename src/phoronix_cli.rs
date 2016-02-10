use article::Article;
use homepage;

pub fn print() {
    let phoronix_articles = Article::get_articles(homepage::offline());
    for article in phoronix_articles.iter().rev() {
        println!("Title:   {}", article.title);
        println!("Link:    https://www.phoronix.com/{}", article.link);
        println!("Details: {}", article.details);
        println!("Summary: {}\n", article.summary);
    }
}