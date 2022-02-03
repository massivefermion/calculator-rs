mod ast;
mod expression;
mod parser;
mod tokenizer;
mod utils;

use crate::ast::AST;
use crate::parser::Parser;
use std::env;
use std::panic;

fn main() {
    panic::set_hook(Box::new(|msg| println!("{}", msg)));

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("not enough input");
    }

    if args[1] == "eval" {
        let mut p = Parser::new(args[2].as_str());
        println!("{}", p.eval());
    } else if args[1] == "ast" {
        let mut ast = AST::new(args[2].as_str());
        println!("{}", serde_json::to_string_pretty(&ast.generate()).unwrap());
    } else {
        panic!("invalid command");
    }
}
