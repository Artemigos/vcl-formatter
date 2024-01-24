use crate::ast::*;
use crate::lexer::Token;

peg::parser! {
    pub grammar vcl<'a>() for [Token<'a>] {
        rule assign_op() -> &'a str
            = [Token::Assign] {"="}
            / [Token::AddAssign] {"+="}
            / [Token::SubtractAssign] {"-="}
            / [Token::MultiplyAssign] {"*="}
            / [Token::DivideAssign] {"/="}

        rule literal() -> Expression<'a>
            = [Token::String(s)] {Expression::Literal(s)}
            / [Token::Duration(s)] {Expression::Literal(s)}
            / [Token::Bytes(s)] {Expression::Literal(s)}
            / [Token::Number(s)] {Expression::Literal(s)}
            / [Token::Bool(s)] {Expression::Literal(s)}

        rule vcl_version() -> TopLevelDeclaration<'a>
            = [Token::Vcl] [Token::Number(n)] [Token::Semicolon] {TopLevelDeclaration::VclVersion(n)}

        rule include() -> &'a str
            = [Token::Include] [Token::String(s)] [Token::Semicolon] {s}

        rule import() -> TopLevelDeclaration<'a>
            = [Token::Import] [Token::Ident(i)] from:([Token::From] [Token::String(s)] {s})? [Token::Semicolon] {
                TopLevelDeclaration::Import { name: i, from }
            }

        rule acl_entry() -> AclEntry<'a>
            = [Token::String(s)] mask:([Token::Divide] [Token::Number(n)] {n})? [Token::Semicolon] {
                AclEntry { value: s, mask }
            }

        rule acl() -> TopLevelDeclaration<'a>
            = [Token::Acl] [Token::Ident(i)] [Token::LBrace] e:acl_entry()* [Token::RBrace] {
                TopLevelDeclaration::Acl { name: i, entries: e }
            }

        rule expression() -> Expression<'a> = precedence!{
            x:(@) [Token::Plus] y:@ {Expression::Binary { left: Box::new(x), op: "+", right: Box::new(y) }}
            x:(@) [Token::Minus] y:@ {Expression::Binary { left: Box::new(x), op: "-", right: Box::new(y) }}
            --
            x:(@) [Token::Multiply] y:@ {Expression::Binary { left: Box::new(x), op: "*", right: Box::new(y) }}
            x:(@) [Token::Divide] y:@ {Expression::Binary { left: Box::new(x), op: "/", right: Box::new(y) }}
            --
            x:(@) [Token::And] y:@ {Expression::Binary { left: Box::new(x), op: "&&", right: Box::new(y) }}
            --
            x:(@) [Token::Or] y:@ {Expression::Binary { left: Box::new(x), op: "||", right: Box::new(y) }}
            --
            x:(@) [Token::Equals] y:@ {Expression::Binary { left: Box::new(x), op: "==", right: Box::new(y) }}
            x:(@) [Token::NotEquals] y:@ {Expression::Binary { left: Box::new(x), op: "!=", right: Box::new(y) }}
            x:(@) [Token::Matches] y:@ {Expression::Binary { left: Box::new(x), op: "~", right: Box::new(y) }}
            x:(@) [Token::NotMatches] y:@ {Expression::Binary { left: Box::new(x), op: "!~", right: Box::new(y) }}
            x:(@) [Token::Greater] y:@ {Expression::Binary { left: Box::new(x), op: ">", right: Box::new(y) }}
            x:(@) [Token::Lesser] y:@ {Expression::Binary { left: Box::new(x), op: "<", right: Box::new(y) }}
            x:(@) [Token::GreaterEquals] y:@ {Expression::Binary { left: Box::new(x), op: ">=", right: Box::new(y) }}
            x:(@) [Token::LesserEquals] y:@ {Expression::Binary { left: Box::new(x), op: "<=", right: Box::new(y) }}
            --
            [Token::Negate] x:@ {Expression::Neg(Box::new(x))}
            --
            l:literal() {l}
            e:ident_call_expr() {Expression::IdentCall(e)}
            [Token::Ident(i)] {Expression::Ident(i)}
            [Token::LParen] e:expression() [Token::RParen] {e}
        }

        rule function_call_arg() -> FunctionCallArg<'a>
            = [Token::Ident(i)] [Token::Assign] e:expression() {FunctionCallArg::Named { name: i, value: e }}
            / e:expression() {FunctionCallArg::Positional(e)}

        rule ident_call_expr() -> IdentCallExpression<'a>
            = [Token::Ident(i)] [Token::LParen] a:function_call_arg()**[Token::Comma] [Token::RParen] {IdentCallExpression { name: i, args: a }}

        rule string_list() -> Vec<&'a str>
            = s:([Token::String(s)] {s})*<2,> {s}

        rule backend_value() -> BackendValue<'a>
            = s:string_list() { BackendValue::StringList(s) }
            / e:expression() { BackendValue::Expression(e) }
            / [Token::LBrace] p:backend_property()* [Token::RBrace] { BackendValue::Composite(p) }

        rule backend_property() -> BackendProperty<'a>
            = [Token::BackendPropIdent(i)] [Token::Assign] v:backend_value() [Token::Semicolon] {
                BackendProperty { name: i, value: v }
            }

        rule backend() -> TopLevelDeclaration<'a>
            = [Token::Backend] [Token::Ident(i)] [Token::LBrace] p:backend_property()* [Token::RBrace] {
                TopLevelDeclaration::Backend { name: i, properties: Some(p) }
            }
            / [Token::Backend] [Token::Ident(i)] [Token::None] [Token::Semicolon] {
                TopLevelDeclaration::Backend { name: i, properties: None }
            }

        rule probe() -> TopLevelDeclaration<'a>
            = [Token::Probe] [Token::Ident(i)] [Token::LBrace] p:backend_property()* [Token::RBrace] {
                TopLevelDeclaration::Probe { name: i, properties: p }
            }

        rule unset_statement() -> Statement<'a>
            = [Token::Unset] [Token::Ident(i)] [Token::Semicolon] {Statement::Unset { ident: i }}

        rule set_statement() -> Statement<'a>
            = [Token::Set] [Token::Ident(i)] op:assign_op() e:expression() [Token::Semicolon] {
                Statement::Set {
                    ident: i,
                    op,
                    expr: e,
                }
            }

        rule body() -> Vec<Statement<'a>>
            = [Token::LBrace] s:statement()* [Token::RBrace] {s}

        rule if_statement() -> Statement<'a>
            = [Token::If] [Token::LParen] c:expression() [Token::RParen] s:body() b1:elseif_statement()* b2:else_statement()? {
                Statement::If {
                    condition: c,
                    body: s,
                    elseifs: b1,
                    else_st: b2,
                }
            }

        rule elseif_statement() -> ElseIfStatement<'a>
            = ([Token::Else] [Token::If] / [Token::ElseIf]) [Token::LParen] c:expression() [Token::RParen] s:body() {
                ElseIfStatement {
                    condition: c,
                    body: s,
                }
            }

        rule else_statement() -> Vec<Statement<'a>>
            = [Token::Else] s:body() {s}

        rule new_statement() -> Statement<'a>
            = [Token::New] [Token::Ident(i)] [Token::Assign] e:ident_call_expr() [Token::Semicolon] {
                Statement::New {
                    name: i,
                    value: e,
                }
            }

        rule call_statement() -> Statement<'a>
            = [Token::Call] [Token::Ident(i)] [Token::Semicolon] {Statement::Call { ident: i }}

        rule return_statement() -> Statement<'a>
            = [Token::Return] [Token::LParen] [Token::Ident(i)] a:([Token::LParen] a:expression()**[Token::Comma] [Token::RParen] {a})? [Token::RParen] [Token::Semicolon] {
                Statement::Return {
                    name: i,
                    args: a,
                }
            }

        rule statement() -> Statement<'a>
            = unset_statement()
            / set_statement()
            / if_statement()
            / new_statement()
            / call_statement()
            / e:ident_call_expr() [Token::Semicolon] {Statement::IdentCall(e)}
            / i:include() {Statement::Include(i)}
            / return_statement()

        rule sub() -> TopLevelDeclaration<'a>
            = [Token::Sub] [Token::Ident(i)] s:body() {
                TopLevelDeclaration::Sub {
                    name: i,
                    statements: s,
                }
            }

        rule top_level_declaration() -> TopLevelDeclaration<'a>
            = vcl_version()
            / i:include() {TopLevelDeclaration::Include(i)}
            / import()
            / acl()
            / backend()
            / probe()
            / sub()

        pub rule source_file() -> SourceFile<'a>
            = top_level_declaration()*
    }
}
