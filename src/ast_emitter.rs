use std::io::Write;

use crate::{ast::*, lexer::Token};

#[derive(Debug)]
pub enum E {
    IO(std::io::Error),
    ExpectedInlineToken,
}

impl From<std::io::Error> for E {
    fn from(value: std::io::Error) -> Self {
        E::IO(value)
    }
}

type R = Result<(), E>;

pub struct Emitter<'a> {
    w: &'a mut dyn Write,
    ind: usize,
    ci: usize,
}

impl<'a> Emitter<'a> {
    pub fn new(writer: &'a mut dyn Write, indent: usize) -> Self {
        Self {
            w: writer,
            ind: indent,
            ci: 0,
        }
    }

    pub fn emit(&mut self, sf: &SourceFile) -> R {
        for td in sf {
            self.emit_toplevel_declaration(td)?;
        }
        Ok(())
    }

    fn emit_indent(&mut self) -> R {
        write!(self.w, "{:<i$}", "", i = self.ci * self.ind)?;
        Ok(())
    }

    fn emit_toplevel_declaration(&mut self, td: &TopLevelDeclaration) -> R {
        match td {
            TopLevelDeclaration::VclVersion { number: v, .. } => {
                self.emit_vcl_version(v.content)?;
            }
            TopLevelDeclaration::Import { name, from, .. } => {
                self.emit_import(name.content, from.as_ref())?
            }
            TopLevelDeclaration::Include(i) => self.emit_include(i)?,
            TopLevelDeclaration::Acl { name, entries, .. } => {
                self.emit_acl(name.content, entries)?
            }
            TopLevelDeclaration::Backend(b) => self.emit_backend(b)?,
            TopLevelDeclaration::Probe {
                name, properties, ..
            } => self.emit_probe(name.content, properties)?,
            TopLevelDeclaration::Sub {
                name, statements, ..
            } => self.emit_sub(name.content, statements)?,
        };

        Ok(())
    }

    // fn emit_all_ws(&mut self, ws: &Vec<Token>) -> R {
    //     let mut at_newlines = 0;
    //     for tok in ws {
    //         match tok {
    //             Token::LineComment(c) => {
    //                 at_newlines = 0;
    //                 write!(self.w, "{c}")?;
    //             }
    //             Token::MultilineComment(c) => {
    //                 at_newlines = 0;
    //                 write!(self.w, "{c}")?;
    //             }
    //             Token::InlineCCode(c) => {
    //                 at_newlines = 0;
    //                 write!(self.w, "{c}")?;
    //             }
    //             Token::Newline(_) => {
    //                 at_newlines += 1;
    //                 if at_newlines <= 2 {
    //                     writeln!(self.w)?;
    //                 }
    //             }
    //             _ => {
    //                 return Err(E::ExpectedInlineToken);
    //             }
    //         };
    //     }
    //     Ok(())
    // }

    fn emit_vcl_version(&mut self, v: &str) -> R {
        writeln!(self.w, "vcl {v};")?;
        Ok(())
    }

    fn emit_import(&mut self, name: &str, from: Option<&FromData>) -> R {
        match from {
            Some(FromData { value: f, .. }) => {
                writeln!(self.w, "import {name} from {};", f.content)?
            }
            None => writeln!(self.w, "import {name};")?,
        };
        Ok(())
    }

    fn emit_include(&mut self, inc: &IncludeData) -> R {
        writeln!(self.w, "include {};", inc.name.content)?;
        Ok(())
    }

    fn emit_acl(&mut self, name: &str, entries: &Vec<AclEntry>) -> R {
        writeln!(self.w, "acl {name} {{")?;
        self.ci += 1;
        for entry in entries {
            self.emit_acl_entry(entry)?;
        }
        self.ci -= 1;
        writeln!(self.w, "}}")?;
        Ok(())
    }

    fn emit_acl_entry(&mut self, e: &AclEntry) -> R {
        let v = e.value.content;
        self.emit_indent()?;
        match &e.mask {
            Some(m) => writeln!(self.w, "{v}/{};", m.mask.content)?,
            None => writeln!(self.w, "{v};")?,
        };
        Ok(())
    }

    fn emit_probe(&mut self, name: &str, properties: &Vec<BackendProperty>) -> R {
        writeln!(self.w, "probe {name} {{")?;
        self.ci += 1;
        for prop in properties {
            self.emit_backend_property(prop.name.content, &prop.value)?;
        }
        self.ci -= 1;
        writeln!(self.w, "}}")?;
        Ok(())
    }

    fn emit_backend_property(&mut self, name: &str, value: &BackendValue) -> R {
        self.emit_indent()?;
        write!(self.w, "{name} =")?;
        match &value {
            BackendValue::Expression(e) => {
                write!(self.w, " ")?;
                self.emit_expression(e)?;
                writeln!(self.w, ";")?;
            }
            BackendValue::Composite { properties, .. } => {
                writeln!(self.w, " {{")?;
                self.ci += 1;
                for prop in properties {
                    self.emit_backend_property(prop.name.content, &prop.value)?;
                }
                self.ci -= 1;
                self.emit_indent();
                writeln!(self.w, "}}")?;
            }
            BackendValue::StringList(l) => {
                self.ci += 1;
                for val in l {
                    writeln!(self.w)?;
                    self.emit_indent();
                    write!(self.w, "{}", val.content)?;
                }
                self.ci -= 1;
                writeln!(self.w, ";")?;
            }
        };
        Ok(())
    }

    fn emit_expression(&mut self, expr: &Expression) -> R {
        match expr {
            Expression::Ident(i) => write!(self.w, "{}", i.content)?,
            Expression::Literal(l) => write!(self.w, "{}", l.content)?,
            Expression::Neg { expr, .. } => {
                write!(self.w, "!")?;
                self.emit_expression(expr)?;
            }
            Expression::Binary { left, op, right } => {
                self.emit_expression(left)?;
                write!(self.w, " {} ", op.content)?;
                self.emit_expression(right)?;
            }
            Expression::IdentCall(e) => {
                self.emit_ident_call(e)?;
            }
            Expression::Parenthesized {
                lparen,
                expr,
                rparen,
            } => {
                write!(self.w, "(")?;
                self.emit_expression(expr)?;
                write!(self.w, ")")?;
            }
        };
        Ok(())
    }

    fn emit_ident_call(&mut self, e: &IdentCallExpression) -> R {
        let n = e.name.content;
        write!(self.w, "{n}(")?;
        let mut first = true;
        for arg in &e.args {
            if first {
                first = false;
            } else {
                write!(self.w, ", ")?;
            };
            match arg {
                FunctionCallArg::Named { name, value, .. } => {
                    write!(self.w, "{} = ", name.content)?;
                    self.emit_expression(value)?;
                }
                FunctionCallArg::Positional(p) => self.emit_expression(p)?,
            };
        }
        write!(self.w, ")")?;
        Ok(())
    }

    fn emit_backend(&mut self, b: &BackendData) -> R {
        match b {
            BackendData::None { name, .. } => writeln!(self.w, "backend {} none;", name.content)?,
            BackendData::Defined {
                name, properties, ..
            } => {
                writeln!(self.w, "backend {} {{", name.content)?;
                self.ci += 1;
                for prop in properties {
                    self.emit_backend_property(prop.name.content, &prop.value)?;
                }
                self.ci -= 1;
                writeln!(self.w, "}}")?;
            }
        };
        Ok(())
    }

    fn emit_sub(&mut self, name: &str, statements: &Vec<Statement>) -> R {
        writeln!(self.w, "sub {name} {{")?;
        self.ci += 1;
        for st in statements {
            self.emit_statement(st)?;
        }
        self.ci -= 1;
        writeln!(self.w, "}}")?;
        Ok(())
    }

    fn emit_statement(&mut self, st: &Statement) -> R {
        self.emit_indent();
        match st {
            Statement::Set {
                ident, op, expr, ..
            } => {
                write!(self.w, "set {} {} ", ident.content, op.content)?;
                self.emit_expression(expr)?;
                writeln!(self.w, ";")?;
            }
            Statement::Unset { ident, .. } => writeln!(self.w, "unset {};", ident.content)?,
            Statement::Call { ident, .. } => writeln!(self.w, "call {};", ident.content)?,
            Statement::IdentCall(i) => {
                self.emit_ident_call(i)?;
                writeln!(self.w, ";")?;
            }
            Statement::If {
                condition,
                body,
                elseifs,
                else_st,
                ..
            } => {
                write!(self.w, "if (")?;
                self.emit_expression(condition)?;
                writeln!(self.w, ") {{")?;
                self.ci += 1;
                for st in body {
                    self.emit_statement(st)?;
                }
                self.ci -= 1;
                for ei in elseifs {
                    self.emit_indent();
                    write!(self.w, "}} else if (")?;
                    self.emit_expression(&ei.condition)?;
                    writeln!(self.w, ") {{")?;
                    self.ci += 1;
                    for st in &ei.body {
                        self.emit_statement(st);
                    }
                    self.ci -= 1;
                }
                if let Some(e) = else_st {
                    self.emit_indent();
                    writeln!(self.w, "}} else {{")?;
                    self.ci += 1;
                    for st in &e.body {
                        self.emit_statement(st);
                    }
                    self.ci -= 1;
                }
                self.emit_indent();
                writeln!(self.w, "}}")?;
            }
            Statement::Return { name, args, .. } => {
                write!(self.w, "return ({}", name.content)?;
                if let Some(args) = args {
                    write!(self.w, "(")?;
                    let mut first = true;
                    for arg in &args.args {
                        if first {
                            first = false;
                        } else {
                            write!(self.w, ", ")?;
                        };
                        self.emit_expression(arg)?;
                    }
                    write!(self.w, ")")?;
                }
                writeln!(self.w, ");")?;
            }
            Statement::New { name, value, .. } => {
                write!(self.w, "new {} = ", name.content)?;
                self.emit_ident_call(value)?;
                writeln!(self.w, ";")?;
            }
            Statement::Include(i) => self.emit_include(i)?,
        };
        Ok(())
    }
}
