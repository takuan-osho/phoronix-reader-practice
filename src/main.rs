extern crate hyper;
extern crate select;
extern crate term;
extern crate getopts;
mod article;
mod homepage;
mod phoronix_cli;
mod linesplit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optflag("n", "no-color", "prints without colors");
    opts.optflag("h", "help", "show this information");
    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("h") { print_help(); return; }
    match matches.opt_present("n") {
        true => phoronix_cli::print(),
        false => phoronix_cli::print_colored(),
    };
    phoronix_cli::print_colored();
}

fn print_help() {
    println!("Prints the latest information from Phoronix.");
    println!("    -n, --no-color : prints without colors");
    println!("    -h, --help     : show this information");
}