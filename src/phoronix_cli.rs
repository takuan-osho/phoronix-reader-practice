use article::Article;
use homepage;
use linesplit;
use term;

pub fn print_colored() {
    let phoronix_articles = Article::get_articles(homepage::offline());
    let mut terminal = term::stdout().unwrap();
    for article in phoronix_articles.iter().rev() {
        println!("Title:   ");
        terminal.fg(term::color::BRIGHT_GREEN).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        println!("{}", article.title);
        terminal.reset().unwrap();
        println!("Link:    ");
        terminal.fg(term::color::BRIGHT_CYAN).unwrap();
        println!("https://www.phoronix.com/{}", article.link);
        terminal.reset().unwrap();
        println!("Details: {}\nSummary:", article.details);
        for line in linesplit::split_by_chars(&article.summary, 77).iter() {
            print!(" - ");
            terminal.attr(term::Attr::Bold).unwrap();
            println!("{}", line);
            terminal.reset().unwrap();
        }
        println!("");
    }
}