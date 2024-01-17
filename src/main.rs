#![allow(unused)]

// mod emitter;
mod lexer;
// mod visitor;

use std::io::Read;

use clap::Parser as ClapParser;
use logos::Logos;
use tree_sitter::Parser;
use tree_sitter_vcl;

// #[cfg(test)]
// const EXAMPLE: &[u8] = include_bytes!("../example.vcl");

/// Formatter for VCL code
#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// VCL file to format
    #[arg()]
    file: String,

    /// test number
    #[arg(short, long)]
    test: u8,
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

    if args.test == 0 {
    } else if args.test == 1 {
        let lang = tree_sitter_vcl::language();
        let mut parser = Parser::new();
        parser.set_language(lang).unwrap();
        let tree = parser.parse(&data, None).unwrap();

        // let mut stdout = std::io::stdout().lock();
        // let mut e = emitter::StandardEmitter::new(&mut stdout, args.indent);
        // visitor::visit_tree(&tree, &data, &mut e);
    } else if args.test == 2 {
        let data_str = std::str::from_utf8(&data).unwrap();
        let mut lex = lexer::Token::lexer(data_str);
        // loop {
        //     let token = lex.next();
        //     if let Some(Ok(tok)) = token {
        //         println!("{:?} {}", tok, lex.slice());
        //     } else {
        //         if let Some(Err(_)) = token {
        //             println!("!Error!");
        //         }
        //         break;
        //     }
        // }
    } else {
        panic!("unknown test number: {}", args.test);
    }
}
