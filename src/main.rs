#![allow(unused)]
#![warn(unused_results)]

mod ast;
mod lexer;
mod parser;

use std::io::Read;

use clap::Parser as ClapParser;
use logos::Logos;

#[cfg(test)]
const EXAMPLE: &[u8] = include_bytes!("../example.vcl");

/// Formatter for VCL code
#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// VCL file to format
    #[arg()]
    file: String,

    /// Number of spaces to use for indentation
    #[arg(short, long, default_value_t = 4)]
    indent: usize,
}

fn main() {
    let args = Args::parse();
    let data = if args.file == "-" {
        let mut buf = Vec::new();
        let _ = std::io::stdin().lock().read_to_end(&mut buf);
        buf
    } else {
        std::fs::read(args.file.as_str()).unwrap()
    };

    let data_str = std::str::from_utf8(&data).unwrap();
    let mut lex = lexer::Token::lexer(data_str);

    loop {
        let token = lex.next();
        if let Some(Ok(tok)) = token {
            let data = lexer::get_token_data(tok).unwrap();
            print!("{}{}", data.pre_trivia, data.content);
        } else {
            if let Some(Err(_)) = token {
                panic!("lexing failed");
            }
            break;
        }
    }

    let final_trivia = &lex.source()[lex.extras.last_token_end..];
    print!("{}", final_trivia);
}
