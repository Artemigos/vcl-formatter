#![allow(unused)]

mod ast;
mod emitter;
mod lexer;
mod visitor;

use std::io::Read;

use clap::Parser as ClapParser;
use emitter::Emitter;
use logos::Logos;
use tree_sitter::Parser;
use tree_sitter_vcl;

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

    /// test number
    #[arg(short, long)]
    test: u8,

    /// output enabled
    #[arg(short, long, default_value_t = false)]
    print: bool,
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

        if args.print {
            let mut stdout = std::io::stdout().lock();
            let mut e = emitter::StandardEmitter::new(&mut stdout, args.indent);
            visitor::visit_tree(&tree, &data, &mut e);
        }
    } else if args.test == 2 {
        let data_str = std::str::from_utf8(&data).unwrap();
        let mut lex = lexer::Token::lexer(data_str);
        if args.print {
            let mut stdout = std::io::stdout().lock();
            let mut e = emitter::StandardEmitter::new(&mut stdout, args.indent);
            let mut how_many_newlines: usize = 0;

            loop {
                let token = lex.next();
                if let Some(Ok(tok)) = token {
                    if how_many_newlines > 0 && tok != lexer::Token::Newline {
                        e.newlines(how_many_newlines);
                        how_many_newlines = 0;
                    }
                    match tok {
                        lexer::Token::Acl => e.acl_keyword(),
                        lexer::Token::Vcl => e.vcl_keyword(),
                        lexer::Token::Import => e.import_keyword(),
                        lexer::Token::Include => e.include_keyword(),
                        lexer::Token::From => e.from_keyword(),
                        lexer::Token::Probe => e.probe_keyword(),
                        lexer::Token::Backend => e.backend_keyword(),
                        lexer::Token::None => e.none_keyword(),
                        lexer::Token::Sub => e.sub_keyword(),
                        lexer::Token::Set => e.set_keyword(),
                        lexer::Token::Call => e.call_keyword(),
                        lexer::Token::Unset => e.unset_keyword(),
                        lexer::Token::If => e.if_keyword(),
                        lexer::Token::Else => e.else_keyword(),
                        lexer::Token::Return => e.return_keyword(),
                        lexer::Token::New => e.new_keyword(),
                        lexer::Token::Number => e.number(lex.slice()),
                        lexer::Token::Semicolon => e.semicolon(),
                        lexer::Token::String => e.string(lex.slice()),
                        lexer::Token::Ident => e.ident(lex.slice()),
                        lexer::Token::LBracket => e.body_start(),
                        lexer::Token::RBracket => e.body_end(),
                        lexer::Token::LParen => e.l_paren(),
                        lexer::Token::RParen => e.r_paren(),
                        lexer::Token::Dot => e.prefix_operator("."),
                        lexer::Token::Negate => e.prefix_operator("!"),
                        lexer::Token::Assign => e.infix_operator("="),
                        lexer::Token::Plus => e.infix_operator("+"),
                        lexer::Token::Minus => e.infix_operator("-"),
                        lexer::Token::Multiply => e.infix_operator("*"),
                        lexer::Token::Divide => e.infix_operator("/"),
                        lexer::Token::Comma => e.comma(),
                        lexer::Token::Or => e.infix_operator("||"),
                        lexer::Token::And => e.infix_operator("&&"),
                        lexer::Token::Equals => e.infix_operator("=="),
                        lexer::Token::NotEquals => e.infix_operator("!="),
                        lexer::Token::Matches => e.infix_operator("~"),
                        lexer::Token::Greater => e.infix_operator(">"),
                        lexer::Token::Lesser => e.infix_operator("<"),
                        lexer::Token::GreaterEquals => e.infix_operator(">="),
                        lexer::Token::LesserEquals => e.infix_operator("<="),
                        lexer::Token::Increment => todo!(),
                        lexer::Token::Decrement => todo!(),
                        lexer::Token::ShiftLeft => e.infix_operator("<<"),
                        lexer::Token::ShiftRight => e.infix_operator(">>"),
                        lexer::Token::AddAssign => e.infix_operator("+="),
                        lexer::Token::SubtractAssign => e.infix_operator("-="),
                        lexer::Token::MultiplyAssign => e.infix_operator("*="),
                        lexer::Token::DivideAssign => e.infix_operator("/="),
                        lexer::Token::NotMatches => e.infix_operator("!~"),
                        lexer::Token::Modulo => e.infix_operator("%"),
                        lexer::Token::BitwiseAnd => e.infix_operator("&"),
                        lexer::Token::BitwiseOr => e.infix_operator("|"),
                        lexer::Token::LineComment => e.comment(lex.slice()),
                        lexer::Token::MultilineComment => e.comment(lex.slice()),
                        lexer::Token::InlineCCode => todo!(),
                        lexer::Token::Newline => how_many_newlines += 1,
                        x => panic!("unknown token: {:?} \"{}\"", x, lex.slice()),
                    }
                } else {
                    if let Some(Err(_)) = token {
                        panic!("lexing failed");
                    }

                    if how_many_newlines > 0 {
                        e.newlines(how_many_newlines);
                        how_many_newlines = 0;
                    }

                    e.file_end();
                    break;
                }
            }
        }
    } else {
        panic!("unknown test number: {}", args.test);
    }
}
