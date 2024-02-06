
mod ast;
mod ast_emitter;
mod emitter;
mod error;
mod lexer;
mod parser;

use std::io::{Read, Write};

use clap::Parser as ClapParser;
use error::R;

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
    let mut stdout = std::io::stdout().lock();
    process_vcl(data_str, args.indent, &mut stdout).unwrap();
}

fn process_vcl(data: &str, indent: usize, out: &mut dyn Write) -> R {
    let tokens = lexer::lex(data)?;
    let ast = parser::vcl::source_file(&tokens)?;
    let mut emitter = ast_emitter::AstEmitter::new(out, indent);
    emitter.emit(&ast)?;
    Ok(())
}
