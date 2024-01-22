use crate::ast::*;
use crate::lexer::Token;

peg::parser! {
    pub grammar vcl<'a>() for [Token<'a>] {
        rule assign_op() -> AssignOperator
            = [Token::Assign] { AssignOperator::Assign }
            / [Token::AddAssign] { AssignOperator::AddAssign }
            / [Token::SubtractAssign] { AssignOperator::SubAssign }
            / [Token::MultiplyAssign] { AssignOperator::MulAssign }
            / [Token::DivideAssign] { AssignOperator::DivAssign }

        rule literal() -> Expression<'a>
            = [Token::String(s)] {Expression::Literal(s)}
            / [Token::Duration(s)] {Expression::Literal(s)}
            / [Token::Bytes(s)] {Expression::Literal(s)}
            / [Token::Number(s)] {Expression::Literal(s)}
            / [Token::Bool(s)] {Expression::Literal(s)}

        rule vcl_version() -> TopLevelDeclaration<'a>
            = [Token::Vcl] [Token::Number(n)] [Token::Semicolon] {TopLevelDeclaration::VclVersion(n)}

        rule include() -> TopLevelDeclaration<'a>
            = [Token::Include] [Token::String(s)] [Token::Semicolon] {TopLevelDeclaration::Include(s)}

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

        rule expression() -> Expression<'a>
            = literal()
            / e:ident_call_expr() {Expression::IdentCall(e)}
            / [Token::Ident(i)] {Expression::Ident(i)}
            / [Token::LParen] e:expression() [Token::RParen] {e}
            // TODO: binary_expression
            / [Token::Negate] e:expression() {Expression::Neg(Box::new(e))}

        rule function_call_arg() -> FunctionCallArg<'a>
            = e:expression() {FunctionCallArg::Positional(e)}
            / [Token::Ident(i)] [Token::Assign] e:expression() {FunctionCallArg::Named { name: i, value: e }}

        rule ident_call_expr() -> IdentCallExpression<'a>
            = [Token::Ident(i)] [Token::LParen] a:function_call_arg()**[Token::Comma] [Token::RParen] {IdentCallExpression { name: i, args: a }}

        rule string_list() -> Vec<&'a str>
            = s:([Token::String(s)] {s})*<2,> {s}

        rule backend_value() -> BackendValue<'a>
            = e:expression() { BackendValue::Expression(e) }
            / s:string_list() { BackendValue::StringList(s) }
            / [Token::LBrace] p:backend_property()* [Token::RBrace] { BackendValue::Composite(p) }

        rule backend_property() -> BackendProperty<'a>
            = [Token::BackendPropIdent(i)] [Token::Assign] v:backend_value()* [Token::Semicolon] {
                BackendProperty { name: i, values: v }
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

        rule statement() -> Statement<'a>
            = unset_statement()
            / set_statement()
            / if_statement()
            / new_statement()
            / call_statement()
            / e:ident_call_expr() {Statement::IdentCall(e)}
            // TODO: include
            // TODO: return

        rule sub() -> TopLevelDeclaration<'a>
            = [Token::Sub] [Token::Ident(i)] s:body() {
                TopLevelDeclaration::Sub {
                    name: i,
                    statements: s,
                }
            }

        rule top_level_declaration() -> TopLevelDeclaration<'a>
            = vcl_version()
            / include()
            / import()
            / acl()
            / backend()
            / probe()
            / sub()

        pub rule source_file() -> SourceFile<'a>
            = top_level_declaration()*
    }
}
