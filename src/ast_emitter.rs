use std::io::Write;

use crate::ast::*;

pub fn emit_file(sf: SourceFile, w: &mut dyn Write) {
    for td in sf {
        emit_toplevel_declaration(td, w);
    }
}

fn emit_toplevel_declaration(td: TopLevelDeclaration, w: &mut dyn Write) {
    match td {
        TopLevelDeclaration::VclVersion(v) => emit_vcl_version(v, w),
        TopLevelDeclaration::Import { name, from } => todo!(),
        TopLevelDeclaration::Include(_) => todo!(),
        TopLevelDeclaration::Acl { name, entries } => todo!(),
        TopLevelDeclaration::Backend { name, properties } => todo!(),
        TopLevelDeclaration::Probe { name, properties } => todo!(),
        TopLevelDeclaration::Sub { name, statements } => todo!(),
    }
}

fn emit_vcl_version(v: &str, w: &mut dyn Write) {
    writeln!(w, "vcl {v};");
}
