extern crate hyper;
extern crate select;
mod article;
mod homepage;
mod phoronix_cli;
mod linesplit;

fn main() {
    phoronix_cli::print();
}