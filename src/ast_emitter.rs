use std::io::Write;

use crate::{
    ast::*,
    emitter::Emitter,
    lexer::{lex_trivia, TokenData, TriviaToken},
};

#[derive(Debug)]
pub enum E {
    IO(std::io::Error),
    LexingWhitespaceFailed,
}

impl From<std::io::Error> for E {
    fn from(value: std::io::Error) -> Self {
        E::IO(value)
    }
}

impl From<()> for E {
    fn from(_: ()) -> Self {
        E::LexingWhitespaceFailed
    }
}

type R = Result<(), E>;

pub struct AstEmitter<'a> {
    e: crate::emitter::StandardEmitter<'a>,
}

impl<'a> AstEmitter<'a> {
    pub fn new(writer: &'a mut dyn Write, indent: usize) -> Self {
        let e = crate::emitter::StandardEmitter::new(writer, indent);
        Self { e }
    }

    pub fn emit(&mut self, sf: &SourceFile) -> R {
        for td in &sf.declarations {
            self.emit_toplevel_declaration(td)?;
        }
        self.e.file_end();
        Ok(())
    }

    fn emit_all_trivia(&mut self, token: &TokenData) -> R {
        let tokens = lex_trivia(token.pre_trivia)?;
        let mut curr_lines = 0;
        for t in &tokens {
            match t {
                TriviaToken::LineComment(s)
                | TriviaToken::MultilineComment(s)
                | TriviaToken::InlineCCode(s) => {
                    if curr_lines > 0 {
                        self.e.newlines(curr_lines);
                        curr_lines = 0;
                    }
                    self.e.comment(s);
                }
                TriviaToken::Newline => curr_lines += 1,
            }
        }
        if curr_lines > 0 {
            self.e.newlines(curr_lines);
        }
        Ok(())
    }

    fn emit_comments(&mut self, token: &TokenData) -> R {
        let tokens = lex_trivia(token.pre_trivia)?;
        for t in &tokens {
            match t {
                TriviaToken::LineComment(s)
                | TriviaToken::MultilineComment(s)
                | TriviaToken::InlineCCode(s) => self.e.comment(s),
                TriviaToken::Newline => {}
            }
        }
        Ok(())
    }

    fn emit_toplevel_declaration(&mut self, td: &TopLevelDeclaration) -> R {
        match td {
            TopLevelDeclaration::VclVersion { vcl, number, semi } => {
                self.emit_vcl_version(vcl, number, semi)?
            }
            TopLevelDeclaration::Import {
                import,
                name,
                from,
                semi,
            } => self.emit_import(import, name, from.as_ref(), semi)?,
            TopLevelDeclaration::Include(i) => self.emit_include(i)?,
            TopLevelDeclaration::Acl {
                acl,
                name,
                lbrace,
                entries,
                rbrace,
            } => self.emit_acl(acl, name, lbrace, entries, rbrace)?,
            TopLevelDeclaration::Backend(b) => self.emit_backend(b)?,
            TopLevelDeclaration::Probe {
                probe,
                name,
                lbrace,
                properties,
                rbrace,
            } => self.emit_probe(probe, name, lbrace, properties, rbrace)?,
            TopLevelDeclaration::Sub {
                sub,
                name,
                lbrace,
                statements,
                rbrace,
            } => self.emit_sub(sub, name, lbrace, statements, rbrace)?,
        };

        Ok(())
    }

    fn emit_vcl_version(&mut self, vcl: &TokenData, number: &TokenData, semi: &TokenData) -> R {
        self.emit_all_trivia(vcl)?;
        self.emit_comments(number)?;
        self.emit_comments(semi)?;

        self.e.vcl_keyword();
        self.e.number(number.content);
        self.e.semicolon();

        Ok(())
    }

    fn emit_import(
        &mut self,
        import: &TokenData,
        name: &TokenData,
        from: Option<&FromData>,
        semi: &TokenData,
    ) -> R {
        self.emit_all_trivia(import)?;
        self.emit_comments(name)?;
        if let Some(FromData { value, from }) = from {
            self.emit_comments(from)?;
            self.emit_comments(value)?;
        }
        self.emit_comments(semi)?;

        self.e.import_keyword();
        self.e.ident(name.content);
        if let Some(FromData { value, .. }) = from {
            self.e.from_keyword();
            self.e.string(value.content);
        }
        self.e.semicolon();

        Ok(())
    }

    fn emit_include(&mut self, inc: &IncludeData) -> R {
        self.emit_all_trivia(&inc.include)?;
        self.emit_comments(&inc.name)?;
        self.emit_comments(&inc.semi)?;

        self.e.include_keyword();
        self.e.string(inc.name.content);
        self.e.semicolon();

        Ok(())
    }

    fn emit_acl(
        &mut self,
        acl: &TokenData,
        name: &TokenData,
        lbrace: &TokenData,
        entries: &Vec<AclEntry>,
        rbrace: &TokenData,
    ) -> R {
        self.emit_all_trivia(acl)?;
        self.emit_comments(name)?;
        self.emit_comments(lbrace)?;

        self.e.acl_keyword();
        self.e.ident(name.content);
        self.e.body_start();
        for entry in entries {
            self.emit_acl_entry(entry)?;
        }

        self.emit_all_trivia(rbrace)?;
        self.e.body_end();

        Ok(())
    }

    fn emit_acl_entry(&mut self, e: &AclEntry) -> R {
        self.emit_all_trivia(&e.value)?;
        if let Some(m) = &e.mask {
            self.emit_comments(&m.op)?;
            self.emit_comments(&m.mask)?;
        }
        self.emit_comments(&e.semi)?;

        self.e.string(e.value.content);
        if let Some(m) = &e.mask {
            self.e.infix_operator("/");
            self.e.number(m.mask.content);
        }
        self.e.semicolon();

        Ok(())
    }

    fn emit_probe(
        &mut self,
        probe: &TokenData,
        name: &TokenData,
        lbrace: &TokenData,
        properties: &Vec<BackendProperty>,
        rbrace: &TokenData,
    ) -> R {
        self.emit_all_trivia(probe)?;
        self.emit_comments(name)?;
        self.emit_comments(lbrace)?;

        self.e.probe_keyword();
        self.e.ident(name.content);
        self.e.body_start();
        for prop in properties {
            self.emit_backend_property(&prop.name, &prop.op, &prop.value)?;
        }

        self.emit_all_trivia(rbrace)?;
        self.e.body_end();

        Ok(())
    }

    fn emit_backend_property(
        &mut self,
        name: &TokenData,
        op: &TokenData,
        value: &BackendValue,
    ) -> R {
        self.emit_all_trivia(name)?;
        self.emit_comments(op)?;
        match &value {
            BackendValue::Expression { expr, semi } => {
                self.emit_expression_comments(expr)?;
                self.emit_comments(semi)?;
            }
            BackendValue::Composite { lbrace, .. } => self.emit_comments(lbrace)?,
            BackendValue::StringList { strings, semi } => {
                for val in strings {
                    self.emit_comments(val)?;
                }
                self.emit_comments(semi)?;
            }
        }

        self.e.ident(name.content);
        self.e.infix_operator("=");
        match &value {
            BackendValue::Expression { expr, .. } => {
                self.emit_expression(expr)?;
                self.e.semicolon();
            }
            BackendValue::Composite {
                properties, rbrace, ..
            } => {
                self.e.body_start();
                for prop in properties {
                    self.emit_backend_property(&prop.name, &prop.op, &prop.value)?;
                }

                self.emit_all_trivia(rbrace)?;
                self.e.body_end();
            }
            BackendValue::StringList { strings, .. } => {
                self.e.hint_string_list_start();
                for val in strings {
                    self.e.string(val.content);
                }
                self.e.semicolon();
            }
        };

        Ok(())
    }

    fn emit_expression_comments(&mut self, expr: &Expression) -> R {
        match expr {
            Expression::Ident(i) => self.emit_comments(i)?,
            Expression::Literal(l) => self.emit_comments(l)?,
            Expression::Neg { op, expr } => {
                self.emit_comments(op)?;
                self.emit_expression_comments(expr)?;
            }
            Expression::Binary { left, op, right } => {
                self.emit_expression_comments(left)?;
                self.emit_comments(op)?;
                self.emit_expression_comments(right)?;
            }
            Expression::IdentCall(c) => self.emit_ident_call_trivia(c, false)?,
            Expression::Parenthesized {
                lparen,
                expr,
                rparen,
            } => {
                self.emit_comments(lparen)?;
                self.emit_expression_comments(expr)?;
                self.emit_comments(rparen)?;
            }
        }
        Ok(())
    }

    fn emit_ident_call_trivia(
        &mut self,
        expr: &IdentCallExpression,
        emit_all_from_first: bool,
    ) -> R {
        if emit_all_from_first {
            self.emit_all_trivia(&expr.name)?;
        } else {
            self.emit_comments(&expr.name)?;
        }
        self.emit_comments(&expr.lparen)?;
        for arg in &expr.args {
            // TODO: commas
            match arg {
                FunctionCallArg::Named { name, op, value } => {
                    self.emit_comments(name)?;
                    self.emit_comments(op)?;
                    self.emit_expression_comments(value)?;
                }
                FunctionCallArg::Positional(e) => {
                    self.emit_expression_comments(e)?;
                }
            }
        }
        self.emit_comments(&expr.rparen)?;
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
            Expression::Parenthesized { expr, .. } => {
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
        match b {
            BackendData::None {
                backend,
                name,
                none,
                semi,
            } => {
                self.emit_all_trivia(backend)?;
                self.emit_comments(name)?;
                self.emit_comments(none)?;
                self.emit_comments(semi)?;

                self.e.backend_keyword();
                self.e.ident(name.content);
                self.e.none_keyword();
                self.e.semicolon();
            }
            BackendData::Defined {
                backend,
                name,
                lbrace,
                properties,
                rbrace,
            } => {
                self.emit_all_trivia(backend)?;
                self.emit_comments(name)?;
                self.emit_comments(lbrace)?;

                self.e.backend_keyword();
                self.e.ident(name.content);
                self.e.body_start();
                for prop in properties {
                    self.emit_backend_property(&prop.name, &prop.op, &prop.value)?;
                }

                self.emit_all_trivia(rbrace)?;
                self.e.body_end();
            }
        };

        Ok(())
    }

    fn emit_sub(
        &mut self,
        sub: &TokenData,
        name: &TokenData,
        lbrace: &TokenData,
        statements: &Vec<Statement>,
        rbrace: &TokenData,
    ) -> R {
        self.emit_all_trivia(sub)?;
        self.emit_comments(name)?;
        self.emit_comments(lbrace)?;

        self.e.sub_keyword();
        self.e.ident(name.content);
        self.e.body_start();
        for st in statements {
            self.emit_statement(st)?;
        }

        self.emit_all_trivia(rbrace)?;
        self.e.body_end();

        Ok(())
    }

    fn emit_statement(&mut self, st: &Statement) -> R {
        match st {
            Statement::Set {
                set,
                ident,
                op,
                expr,
                semi,
            } => {
                self.emit_all_trivia(set)?;
                self.emit_comments(ident)?;
                self.emit_comments(op)?;
                self.emit_expression_comments(expr)?;
                self.emit_comments(semi)?;

                self.e.set_keyword();
                self.e.ident(ident.content);
                self.e.infix_operator(op.content);
                self.emit_expression(expr)?;
                self.e.semicolon();
            }
            Statement::Unset { unset, ident, semi } => {
                self.emit_all_trivia(unset)?;
                self.emit_comments(ident)?;
                self.emit_comments(semi)?;

                self.e.unset_keyword();
                self.e.ident(ident.content);
                self.e.semicolon();
            }
            Statement::Call { call, ident, semi } => {
                self.emit_all_trivia(call)?;
                self.emit_comments(ident)?;
                self.emit_comments(semi)?;

                self.e.call_keyword();
                self.e.ident(ident.content);
                self.e.semicolon();
            }
            Statement::IdentCall { expr, semi } => {
                self.emit_ident_call_trivia(expr, true)?;
                self.emit_comments(semi)?;

                self.emit_ident_call(expr)?;
                self.e.semicolon();
            }
            Statement::If {
                if_t,
                lparen,
                condition,
                rparen,
                lbrace,
                body,
                rbrace,
                elseifs,
                else_st,
            } => {
                self.emit_all_trivia(if_t)?;
                self.emit_comments(lparen)?;
                self.emit_expression_comments(condition)?;
                self.emit_comments(rparen)?;
                self.emit_comments(lbrace)?;

                self.e.if_keyword();
                self.e.l_paren();
                self.emit_expression(condition)?;
                self.e.r_paren();
                self.e.body_start();
                for st in body {
                    self.emit_statement(st)?;
                }
                self.emit_all_trivia(rbrace)?;
                for ei in elseifs {
                    self.e.body_end();
                    self.e.else_keyword();
                    self.e.if_keyword();
                    self.e.l_paren();
                    self.emit_expression(&ei.condition)?;
                    self.e.r_paren();
                    self.e.body_start();

                    for t in &ei.elseif {
                        self.emit_comments(t)?;
                    }
                    self.emit_comments(&ei.lparen)?;
                    self.emit_expression_comments(&ei.condition)?;
                    self.emit_comments(&ei.rparen)?;
                    self.emit_comments(&ei.lbrace)?;

                    for st in &ei.body {
                        self.emit_statement(st)?;
                    }

                    self.emit_all_trivia(&ei.rbrace)?;
                }
                if let Some(e) = else_st {
                    self.e.body_end();
                    self.e.else_keyword();
                    self.e.body_start();

                    self.emit_comments(&e.else_t)?;
                    self.emit_comments(&e.lbrace)?;

                    for st in &e.body {
                        self.emit_statement(st)?;
                    }

                    self.emit_all_trivia(&e.rbrace)?;
                }
                self.e.body_end();
            }
            Statement::Return {
                return_t,
                lparen,
                name,
                args,
                rparen,
                semi,
            } => {
                self.emit_all_trivia(return_t)?;
                self.emit_comments(lparen)?;
                self.emit_comments(name)?;
                if let Some(args) = args {
                    self.emit_comments(&args.lparen)?;
                    for e in &args.args {
                        self.emit_expression_comments(e)?;
                    }
                    self.emit_comments(&args.rparen)?;
                }
                self.emit_comments(rparen)?;
                self.emit_comments(semi)?;

                self.e.return_keyword();
                self.e.l_paren();
                self.e.ident(name.content);
                if let Some(args) = args {
                    self.e.l_paren();
                    let mut first = true;
                    for arg in &args.args {
                        // TODO: commas
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
            Statement::New {
                new,
                name,
                op,
                value,
                semi,
            } => {
                self.emit_all_trivia(new)?;
                self.emit_comments(name)?;
                self.emit_comments(op)?;
                self.emit_ident_call_trivia(value, false)?;
                self.emit_comments(semi)?;

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
