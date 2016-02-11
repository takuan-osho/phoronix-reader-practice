extern crate hyper;
extern crate select;
extern crate term;
mod article;
mod homepage;
mod phoronix_cli;
mod linesplit;

fn main() {
    phoronix_cli::print_colored();
}