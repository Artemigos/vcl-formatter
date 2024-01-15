mod emitter;
mod visitor;

use tree_sitter::Parser;
use tree_sitter_vcl;

#[cfg(test)]
const EXAMPLE: &[u8] = include_bytes!("../example.vcl");

fn main() {
    let args = std::env::args();
    let args: Vec<_> = args.collect();
    assert_eq!(args.len(), 2, "Exactly 1 argument required - the VCL file path.");
    let data = std::fs::read(args[1].as_str()).unwrap();

    let lang = tree_sitter_vcl::language();
    let mut parser = Parser::new();
    parser.set_language(lang).unwrap();
    let tree = parser.parse(&data, None).unwrap();

    let mut stdout = std::io::stdout().lock();
    let mut e = emitter::StandardEmitter::new(&mut stdout, 4);
    visitor::visit_tree(&tree, &data, &mut e);
}
