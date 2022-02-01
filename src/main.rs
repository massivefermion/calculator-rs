mod ast;
mod expression;
mod parser;
mod tokenizer;
mod utils;

use crate::ast::AST;
use crate::parser::Parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("not enough input");
    }

    if args[1] == "eval" {
        let mut p = Parser::new(args[2].as_str());
        println!("{:?}", p.eval());
    } else if args[1] == "ast" {
        let mut ast = AST::new(args[2].as_str());
        println!("{:?}", ast.generate());
    } else {
        panic!("invalid command");
    }
}
