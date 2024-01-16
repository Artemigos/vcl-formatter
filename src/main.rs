mod emitter;
mod visitor;

use std::io::Read;

use clap::Parser as ClapParser;
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
    indent_size: usize,
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

    let lang = tree_sitter_vcl::language();
    let mut parser = Parser::new();
    parser.set_language(lang).unwrap();
    let tree = parser.parse(&data, None).unwrap();

    let mut stdout = std::io::stdout().lock();
    let mut e = emitter::StandardEmitter::new(&mut stdout, args.indent_size);
    visitor::visit_tree(&tree, &data, &mut e);
}
