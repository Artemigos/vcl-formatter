use tree_sitter::{Node, Tree, TreeCursor};

use crate::emitter::Emitter;

struct TreeIter<'a, 'b, 'c> {
    cursor: TreeCursor<'a>,
    do_not_descend: &'b [&'c str],
}

impl<'a, 'b, 'c> Iterator for TreeIter<'a, 'b, 'c> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.do_not_descend.contains(&self.cursor.node().kind())
            && self.cursor.goto_first_child()
        {
            return Some(self.cursor.node());
        }
        loop {
            if self.cursor.goto_next_sibling() {
                return Some(self.cursor.node());
            }
            if !self.cursor.goto_parent() {
                return None;
            }
        }
    }
}

fn get<'a>(node: &Node, source: &'a [u8]) -> &'a str {
    node.utf8_text(source).unwrap()
}

pub fn visit_tree(tree: &Tree, source: &[u8], e: &mut dyn Emitter) {
    let matched = [
        "vcl",
        "include",
        "import",
        "from",
        "probe",
        "backend",
        "none",
        "acl",
        "sub",
        "set",
        "unset",
        "if",
        "else",
        "return",
        "call",
        "{",
        "}",
        ";",
        "(",
        ")",
        "=",
        ".",
        "!",
        "/",
        ",",
        "operator",
        "number",
        "ident",
        "nested_ident",
        "string",
        "literal",
        "varnish_internal_return_methods",
        "COMMENT",
        "ERROR",
    ];

    let mut is_string_list = false;
    let mut is_acl = false;
    let mut is_call = false;
    let mut last_consumed = 0;

    let cursor = tree.walk();
    let v = TreeIter {
        cursor,
        do_not_descend: &["literal", "operator"],
    };
    for node in v {
        let r = node.range();
        let is_match = matched.contains(&node.kind());

        if is_match {
            let subdata = &source[last_consumed..r.start_byte];
            let whitespace = std::str::from_utf8(subdata).unwrap();
            let how_many = whitespace.chars().filter(|x| *x == '\n').count();
            if how_many > 0 {
                e.newlines(how_many);
            }
        }

        match node.kind() {
            "vcl" => e.vcl_keyword(),
            "include" => e.include_keyword(),
            "import" => e.import_keyword(),
            "from" => e.from_keyword(),
            "probe" => e.probe_keyword(),
            "backend" => e.backend_keyword(),
            "none" => e.none_keyword(),
            "acl" => {
                e.acl_keyword();
                is_acl = true;
            }
            "sub" => e.sub_keyword(),
            "set" => e.set_keyword(),
            "unset" => e.unset_keyword(),
            "if" => e.if_keyword(),
            "else" => e.else_keyword(),
            "return" => e.return_keyword(),
            "call" => e.call_keyword(),
            "{" => e.body_start(),
            "}" => {
                e.body_end();
                is_acl = false;
            }
            ";" => {
                e.semicolon();
                is_string_list = false;
            }
            "(" => e.l_paren(),
            ")" => e.r_paren(),
            "=" => e.infix_operator("="),
            "." => e.prefix_operator("."),
            "!" => e.prefix_operator("!"),
            "/" => {
                if is_acl {
                    e.acl_mask_op();
                }
            }
            "," => e.comma(),
            "operator" => e.infix_operator(get(&node, source)),
            "number" => e.number(get(&node, source)),
            "ident" | "nested_ident" => {
                if is_call {
                    e.call_ident(get(&node, source));
                    is_call = false;
                } else {
                    e.ident(get(&node, source));
                }
            }
            "string" => {
                if is_string_list {
                    e.string_list_entry(get(&node, source));
                } else {
                    e.string(get(&node, source));
                }
            }
            "string_list" => {
                is_string_list = true;
            }
            "literal" => e.expression(get(&node, source)),
            "ident_call_expr" => {
                is_call = true;
            }
            "varnish_internal_return_methods" => {
                let val = get(&node, source);
                let spl = val.split_once("(");
                match spl {
                    Some((pre, _)) => {
                        let name = pre.trim();
                        e.varnish_step_keyword(name);
                        last_consumed += name.len();
                    }
                    None => e.varnish_step_keyword(val),
                }
            }
            "COMMENT" => e.comment(get(&node, source)),
            "ERROR" => panic!("syntax error"),
            _ => {}
        }

        if is_match && node.kind() != "varnish_internal_return_methods" {
            last_consumed = r.end_byte;
        }
    }

    e.file_end();
}
