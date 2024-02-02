use crate::ast::*;
use crate::lexer::Token;

peg::parser! {
    pub grammar vcl<'a>() for [Token<'a>] {
        rule _ -> Vec<Token<'a>>
            = ([Token::Newline(_)] / [Token::LineComment(_)] / [Token::MultilineComment(_)] / [Token::InlineCCode(_)])*

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
            = i1:_ [Token::Vcl] i2:_ [Token::Number(n)] i3:_ [Token::Semicolon] {
                TopLevelDeclaration::VclVersion {
                    ws_pre_vcl: i1,
                    ws_pre_number: i2,
                    ws_pre_semi: i3,
                    number: n,
                }
            }

        rule include() -> IncludeData<'a>
            = i1:_ [Token::Include] i2:_ [Token::String(s)] i3:_ [Token::Semicolon] {
                IncludeData {
                    ws_pre_include: i1,
                    ws_pre_name: i2,
                    ws_pre_semi: i3,
                    name: s,
                }
            }

        rule import_from() -> FromData<'a>
            = i1:_ [Token::From] i2:_ [Token::String(s)] {FromData { ws_pre_from: i1, ws_pre_value: i2, value: s }}

        rule import() -> TopLevelDeclaration<'a>
            = i1:_ [Token::Import] i2:_ [Token::Ident(i)] from:import_from()? i3:_ [Token::Semicolon] {
                TopLevelDeclaration::Import { ws_pre_import: i1, ws_pre_name: i2, ws_pre_semi: i3, name: i, from }
            }

        rule acl_entry() -> AclEntry<'a>
            = _ [Token::String(s)] mask:(_ [Token::Divide] _ [Token::Number(n)] {n})? _ [Token::Semicolon] {
                AclEntry { value: s, mask }
            }

        rule acl() -> TopLevelDeclaration<'a>
            = _ [Token::Acl] _ [Token::Ident(i)] _ [Token::LBrace] e:acl_entry()* _ [Token::RBrace] {
                TopLevelDeclaration::Acl { name: i, entries: e }
            }

        rule expression() -> Expression<'a> = precedence!{
            x:(@) _ [Token::Plus] y:@ {Expression::Binary { left: Box::new(x), op: "+", right: Box::new(y) }}
            x:(@) _ [Token::Minus] y:@ {Expression::Binary { left: Box::new(x), op: "-", right: Box::new(y) }}
            --
            x:(@) _ [Token::Multiply] y:@ {Expression::Binary { left: Box::new(x), op: "*", right: Box::new(y) }}
            x:(@) _ [Token::Divide] y:@ {Expression::Binary { left: Box::new(x), op: "/", right: Box::new(y) }}
            --
            x:(@) _ [Token::And] y:@ {Expression::Binary { left: Box::new(x), op: "&&", right: Box::new(y) }}
            --
            x:(@) _ [Token::Or] y:@ {Expression::Binary { left: Box::new(x), op: "||", right: Box::new(y) }}
            --
            x:(@) _ [Token::Equals] y:@ {Expression::Binary { left: Box::new(x), op: "==", right: Box::new(y) }}
            x:(@) _ [Token::NotEquals] y:@ {Expression::Binary { left: Box::new(x), op: "!=", right: Box::new(y) }}
            x:(@) _ [Token::Matches] y:@ {Expression::Binary { left: Box::new(x), op: "~", right: Box::new(y) }}
            x:(@) _ [Token::NotMatches] y:@ {Expression::Binary { left: Box::new(x), op: "!~", right: Box::new(y) }}
            x:(@) _ [Token::Greater] y:@ {Expression::Binary { left: Box::new(x), op: ">", right: Box::new(y) }}
            x:(@) _ [Token::Lesser] y:@ {Expression::Binary { left: Box::new(x), op: "<", right: Box::new(y) }}
            x:(@) _ [Token::GreaterEquals] y:@ {Expression::Binary { left: Box::new(x), op: ">=", right: Box::new(y) }}
            x:(@) _ [Token::LesserEquals] y:@ {Expression::Binary { left: Box::new(x), op: "<=", right: Box::new(y) }}
            --
            [Token::Negate] x:@ {Expression::Neg(Box::new(x))}
            --
            _ l:literal() {l}
            e:ident_call_expr() {Expression::IdentCall(e)}
            _ [Token::Ident(i)] {Expression::Ident(i)}
            _ [Token::LParen] e:expression() _ [Token::RParen] {e}
        }

        rule function_call_arg() -> FunctionCallArg<'a>
            = _ [Token::Ident(i)] _ [Token::Assign] e:expression() {FunctionCallArg::Named { name: i, value: e }}
            / e:expression() {FunctionCallArg::Positional(e)}

        rule ident_call_expr() -> IdentCallExpression<'a>
            = _ [Token::Ident(i)] _ [Token::LParen] a:function_call_arg()**(_ [Token::Comma]) _ [Token::RParen] {IdentCallExpression { name: i, args: a }}

        rule string_list() -> Vec<&'a str>
            = s:(_ [Token::String(s)] {s})*<2,> {s}

        rule backend_value() -> BackendValue<'a>
            = s:string_list() { BackendValue::StringList(s) }
            / e:expression() { BackendValue::Expression(e) }
            / _ [Token::LBrace] p:backend_property()* _ [Token::RBrace] { BackendValue::Composite(p) }

        rule backend_property() -> BackendProperty<'a>
            = _ [Token::BackendPropIdent(i)] _ [Token::Assign] v:backend_value() _ [Token::Semicolon] {
                BackendProperty { name: i, value: v }
            }

        rule backend() -> TopLevelDeclaration<'a>
            = _ [Token::Backend] _ [Token::Ident(i)] _ [Token::LBrace] p:backend_property()* _ [Token::RBrace] {
                TopLevelDeclaration::Backend { name: i, properties: Some(p) }
            }
            / _ [Token::Backend] _ [Token::Ident(i)] _ [Token::None] _ [Token::Semicolon] {
                TopLevelDeclaration::Backend { name: i, properties: None }
            }

        rule probe() -> TopLevelDeclaration<'a>
            = _ [Token::Probe] _ [Token::Ident(i)] _ [Token::LBrace] p:backend_property()* _ [Token::RBrace] {
                TopLevelDeclaration::Probe { name: i, properties: p }
            }

        rule unset_statement() -> Statement<'a>
            = _ [Token::Unset] _ [Token::Ident(i)] _ [Token::Semicolon] {Statement::Unset { ident: i }}

        rule set_statement() -> Statement<'a>
            = _ [Token::Set] _ [Token::Ident(i)] _ op:assign_op() e:expression() _ [Token::Semicolon] {
                Statement::Set {
                    ident: i,
                    op,
                    expr: e,
                }
            }

        rule body() -> Vec<Statement<'a>>
            = _ [Token::LBrace] s:statement()* _ [Token::RBrace] {s}

        rule if_statement() -> Statement<'a>
            = _ [Token::If] _ [Token::LParen] c:expression() _ [Token::RParen] s:body() b1:elseif_statement()* b2:else_statement()? {
                Statement::If {
                    condition: c,
                    body: s,
                    elseifs: b1,
                    else_st: b2,
                }
            }

        rule elseif_statement() -> ElseIfStatement<'a>
            = (_ [Token::Else] _ [Token::If] / _ [Token::ElseIf]) _ [Token::LParen] c:expression() _ [Token::RParen] s:body() {
                ElseIfStatement {
                    condition: c,
                    body: s,
                }
            }

        rule else_statement() -> Vec<Statement<'a>>
            = _ [Token::Else] s:body() {s}

        rule new_statement() -> Statement<'a>
            = _ [Token::New] _ [Token::Ident(i)] _ [Token::Assign] e:ident_call_expr() _ [Token::Semicolon] {
                Statement::New {
                    name: i,
                    value: e,
                }
            }

        rule call_statement() -> Statement<'a>
            = _ [Token::Call] _ [Token::Ident(i)] _ [Token::Semicolon] {Statement::Call { ident: i }}

        rule return_statement() -> Statement<'a>
            = _ [Token::Return] _ [Token::LParen] _ [Token::Ident(i)] a:(_ [Token::LParen] a:expression()**(_ [Token::Comma]) _ [Token::RParen] {a})? _ [Token::RParen] _ [Token::Semicolon] {
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
            / e:ident_call_expr() _ [Token::Semicolon] {Statement::IdentCall(e)}
            / i:include() {Statement::Include(i)}
            / return_statement()

        rule sub() -> TopLevelDeclaration<'a>
            = _ [Token::Sub] _ [Token::Ident(i)] s:body() {
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
            = d:top_level_declaration()* _ {d}
    }
}
