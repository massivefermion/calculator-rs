mod expression;
mod parser;
mod tokenizer;
mod utils;

use crate::parser::Parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("no expression given");
    }
    let mut p = Parser::new(args[1].as_str());
    println!("{}", p.eval());
}
