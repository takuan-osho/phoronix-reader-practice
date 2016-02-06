extern crate hyper;
extern crate select;

fn open_testing() -> &'static str {
    include_str!("phoronix.html")
}

fn main() {
    let phoronix = open_testing();
    println!("{}", phoronix);
}