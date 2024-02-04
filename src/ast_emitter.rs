use std::io::Write;

use crate::{ast::*, emitter::Emitter, lexer::Token};

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

pub struct AstEmitter<'a> {
    e: crate::emitter::StandardEmitter<'a>,
}

impl<'a> AstEmitter<'a> {
    pub fn new(writer: &'a mut dyn Write, indent: usize) -> Self {
        let mut e = crate::emitter::StandardEmitter::new(writer, indent);
        Self { e }
    }

    pub fn emit(&mut self, sf: &SourceFile) -> R {
        for td in &sf.declarations {
            self.emit_toplevel_declaration(td)?;
        }
        self.e.file_end();
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

    fn emit_vcl_version(&mut self, v: &str) -> R {
        self.e.vcl_keyword();
        self.e.number(v);
        self.e.semicolon();
        Ok(())
    }

    fn emit_import(&mut self, name: &str, from: Option<&FromData>) -> R {
        self.e.import_keyword();
        self.e.ident(name);
        if let Some(FromData { value: f, .. }) = from {
            self.e.from_keyword();
            self.e.string(f.content);
        }
        self.e.semicolon();
        Ok(())
    }

    fn emit_include(&mut self, inc: &IncludeData) -> R {
        self.e.include_keyword();
        self.e.string(inc.name.content);
        self.e.semicolon();
        Ok(())
    }

    fn emit_acl(&mut self, name: &str, entries: &Vec<AclEntry>) -> R {
        self.e.acl_keyword();
        self.e.ident(name);
        self.e.body_start();
        for entry in entries {
            self.emit_acl_entry(entry)?;
        }
        self.e.body_end();
        Ok(())
    }

    fn emit_acl_entry(&mut self, e: &AclEntry) -> R {
        self.e.string(e.value.content);
        if let Some(m) = &e.mask {
            self.e.infix_operator("/");
            self.e.number(m.mask.content);
        }
        self.e.semicolon();
        Ok(())
    }

    fn emit_probe(&mut self, name: &str, properties: &Vec<BackendProperty>) -> R {
        self.e.probe_keyword();
        self.e.ident(name);
        self.e.body_start();
        for prop in properties {
            self.emit_backend_property(prop.name.content, &prop.value)?;
        }
        self.e.body_end();
        Ok(())
    }

    fn emit_backend_property(&mut self, name: &str, value: &BackendValue) -> R {
        self.e.ident(name);
        self.e.infix_operator("=");
        match &value {
            BackendValue::Expression(e) => {
                self.emit_expression(e)?;
                self.e.semicolon();
            }
            BackendValue::Composite { properties, .. } => {
                self.e.body_start();
                for prop in properties {
                    self.emit_backend_property(prop.name.content, &prop.value)?;
                }
                self.e.body_end();
            }
            BackendValue::StringList(l) => {
                for val in l {
                    self.e.string(val.content);
                }
                self.e.semicolon();
            }
        };
        Ok(())
    }

    fn emit_expression(&mut self, expr: &Expression) -> R {
        match expr {
            Expression::Ident(i) => self.e.ident(i.content),
            Expression::Literal(l) => self.e.ident(l.content),
            Expression::Neg { expr, .. } => {
                self.e.prefix_operator("!");
                self.emit_expression(expr)?;
            }
            Expression::Binary { left, op, right } => {
                self.emit_expression(left)?;
                self.e.infix_operator(op.content);
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
                self.e.l_paren();
                self.emit_expression(expr)?;
                self.e.r_paren();
            }
        };
        Ok(())
    }

    fn emit_ident_call(&mut self, e: &IdentCallExpression) -> R {
        self.e.ident(e.name.content);
        self.e.l_paren();
        let mut first = true;
        for arg in &e.args {
            if first {
                first = false;
            } else {
                self.e.comma();
            };
            match arg {
                FunctionCallArg::Named { name, value, .. } => {
                    self.e.ident(name.content);
                    self.e.infix_operator("=");
                    self.emit_expression(value)?;
                }
                FunctionCallArg::Positional(p) => self.emit_expression(p)?,
            };
        }
        self.e.r_paren();
        Ok(())
    }

    fn emit_backend(&mut self, b: &BackendData) -> R {
        self.e.backend_keyword();
        match b {
            BackendData::None { name, .. } => {
                self.e.ident(name.content);
                self.e.none_keyword();
                self.e.semicolon();
            }
            BackendData::Defined {
                name, properties, ..
            } => {
                self.e.ident(name.content);
                self.e.body_start();
                for prop in properties {
                    self.emit_backend_property(prop.name.content, &prop.value)?;
                }
                self.e.body_end();
            }
        };
        Ok(())
    }

    fn emit_sub(&mut self, name: &str, statements: &Vec<Statement>) -> R {
        self.e.sub_keyword();
        self.e.ident(name);
        self.e.body_start();
        for st in statements {
            self.emit_statement(st)?;
        }
        self.e.body_end();
        Ok(())
    }

    fn emit_statement(&mut self, st: &Statement) -> R {
        match st {
            Statement::Set {
                ident, op, expr, ..
            } => {
                self.e.set_keyword();
                self.e.ident(ident.content);
                self.e.infix_operator(op.content);
                self.emit_expression(expr)?;
                self.e.semicolon();
            }
            Statement::Unset { ident, .. } => {
                self.e.unset_keyword();
                self.e.ident(ident.content);
                self.e.semicolon();
            }
            Statement::Call { ident, .. } => {
                self.e.call_keyword();
                self.e.ident(ident.content);
                self.e.semicolon();
            }
            Statement::IdentCall(i) => {
                self.emit_ident_call(i)?;
                self.e.semicolon();
            }
            Statement::If {
                condition,
                body,
                elseifs,
                else_st,
                ..
            } => {
                self.e.if_keyword();
                self.e.l_paren();
                self.emit_expression(condition)?;
                self.e.r_paren();
                self.e.body_start();
                for st in body {
                    self.emit_statement(st)?;
                }
                for ei in elseifs {
                    self.e.body_end();
                    self.e.else_keyword();
                    self.e.if_keyword();
                    self.e.l_paren();
                    self.emit_expression(&ei.condition)?;
                    self.e.r_paren();
                    self.e.body_start();
                    for st in &ei.body {
                        self.emit_statement(st);
                    }
                }
                if let Some(e) = else_st {
                    self.e.body_end();
                    self.e.else_keyword();
                    self.e.body_start();
                    for st in &e.body {
                        self.emit_statement(st);
                    }
                }
                self.e.body_end();
            }
            Statement::Return { name, args, .. } => {
                self.e.return_keyword();
                self.e.l_paren();
                self.e.ident(name.content);
                if let Some(args) = args {
                    self.e.l_paren();
                    let mut first = true;
                    for arg in &args.args {
                        if first {
                            first = false;
                        } else {
                            self.e.comma();
                        };
                        self.emit_expression(arg)?;
                    }
                    self.e.r_paren();
                }
                self.e.r_paren();
                self.e.semicolon();
            }
            Statement::New { name, value, .. } => {
                self.e.new_keyword();
                self.e.ident(name.content);
                self.e.infix_operator("=");
                self.emit_ident_call(value)?;
                self.e.semicolon();
            }
            Statement::Include(i) => self.emit_include(i)?,
        };
        Ok(())
    }
}
