#[drive(Debud)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String
}

use text_colorizer::*;

fn print_usage() {
    eprintln!("{} - change occurence of one string into another",
              "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}