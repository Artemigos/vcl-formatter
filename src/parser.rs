use crate::ast::*;
use crate::lexer::{Token, TokenData};

peg::parser! {
    pub grammar vcl<'a>() for [Token<'a>] {
        rule list<I, S>(item: rule<I>, sep: rule<S>) -> DelimitedList<I, S>
            = items:(i:item() s:sep() { (i, s) })* last:item() {
                DelimitedList::WithItems {
                    pairs: items,
                    last_item: last,
                }
            }
            / {DelimitedList::Empty}

        rule acl() -> TokenData<'a> = [Token::Acl(d)] {d}
        rule vcl() -> TokenData<'a> = [Token::Vcl(d)] {d}
        rule import() -> TokenData<'a> = [Token::Import(d)] {d}
        rule include() -> TokenData<'a> = [Token::Include(d)] {d}
        rule from() -> TokenData<'a> = [Token::From(d)] {d}
        rule probe() -> TokenData<'a> = [Token::Probe(d)] {d}
        rule backend() -> TokenData<'a> = [Token::Backend(d)] {d}
        rule none() -> TokenData<'a> = [Token::None(d)] {d}
        rule sub() -> TokenData<'a> = [Token::Sub(d)] {d}
        rule set() -> TokenData<'a> = [Token::Set(d)] {d}
        rule call() -> TokenData<'a> = [Token::Call(d)] {d}
        rule unset() -> TokenData<'a> = [Token::Unset(d)] {d}
        rule if_r() -> TokenData<'a> = [Token::If(d)] {d}
        rule else_r() -> TokenData<'a> = [Token::Else(d)] {d}
        rule elseIf() -> TokenData<'a> = [Token::ElseIf(d)] {d}
        rule return_r() -> TokenData<'a> = [Token::Return(d)] {d}
        rule new() -> TokenData<'a> = [Token::New(d)] {d}
        rule bool_r() -> TokenData<'a> = [Token::Bool(d)] {d}
        rule number() -> TokenData<'a> = [Token::Number(d)] {d}
        rule duration() -> TokenData<'a> = [Token::Duration(d)] {d}
        rule bytes() -> TokenData<'a> = [Token::Bytes(d)] {d}
        rule string() -> TokenData<'a> = [Token::String(d)] {d}
        rule ident() -> TokenData<'a> = [Token::Ident(d)] {d}
        rule backendPropIdent() -> TokenData<'a> = [Token::BackendPropIdent(d)] {d}
        rule semicolon() -> TokenData<'a> = [Token::Semicolon(d)] {d}
        rule lBrace() -> TokenData<'a> = [Token::LBrace(d)] {d}
        rule rBrace() -> TokenData<'a> = [Token::RBrace(d)] {d}
        rule lParen() -> TokenData<'a> = [Token::LParen(d)] {d}
        rule rParen() -> TokenData<'a> = [Token::RParen(d)] {d}
        rule negate() -> TokenData<'a> = [Token::Negate(d)] {d}
        rule assign() -> TokenData<'a> = [Token::Assign(d)] {d}
        rule plus() -> TokenData<'a> = [Token::Plus(d)] {d}
        rule minus() -> TokenData<'a> = [Token::Minus(d)] {d}
        rule multiply() -> TokenData<'a> = [Token::Multiply(d)] {d}
        rule divide() -> TokenData<'a> = [Token::Divide(d)] {d}
        rule comma() -> TokenData<'a> = [Token::Comma(d)] {d}
        rule or() -> TokenData<'a> = [Token::Or(d)] {d}
        rule and() -> TokenData<'a> = [Token::And(d)] {d}
        rule equals() -> TokenData<'a> = [Token::Equals(d)] {d}
        rule notEquals() -> TokenData<'a> = [Token::NotEquals(d)] {d}
        rule matches() -> TokenData<'a> = [Token::Matches(d)] {d}
        rule greater() -> TokenData<'a> = [Token::Greater(d)] {d}
        rule lesser() -> TokenData<'a> = [Token::Lesser(d)] {d}
        rule greaterEquals() -> TokenData<'a> = [Token::GreaterEquals(d)] {d}
        rule lesserEquals() -> TokenData<'a> = [Token::LesserEquals(d)] {d}
        rule increment() -> TokenData<'a> = [Token::Increment(d)] {d}
        rule decrement() -> TokenData<'a> = [Token::Decrement(d)] {d}
        rule shiftLeft() -> TokenData<'a> = [Token::ShiftLeft(d)] {d}
        rule shiftRight() -> TokenData<'a> = [Token::ShiftRight(d)] {d}
        rule addAssign() -> TokenData<'a> = [Token::AddAssign(d)] {d}
        rule subtractAssign() -> TokenData<'a> = [Token::SubtractAssign(d)] {d}
        rule multiplyAssign() -> TokenData<'a> = [Token::MultiplyAssign(d)] {d}
        rule divideAssign() -> TokenData<'a> = [Token::DivideAssign(d)] {d}
        rule notMatches() -> TokenData<'a> = [Token::NotMatches(d)] {d}
        rule modulo() -> TokenData<'a> = [Token::Modulo(d)] {d}
        rule bitwiseAnd() -> TokenData<'a> = [Token::BitwiseAnd(d)] {d}
        rule bitwiseOr() -> TokenData<'a> = [Token::BitwiseOr(d)] {d}
        rule eof() -> TokenData<'a> = [Token::Eof(d)] {d}

        rule assign_op() -> TokenData<'a>
            = assign()
            / addAssign()
            / subtractAssign()
            / multiplyAssign()
            / divideAssign()

        rule literal() -> Expression<'a>
            = s:string() {Expression::Literal(s)}
            / s:duration() {Expression::Literal(s)}
            / s:bytes() {Expression::Literal(s)}
            / s:number() {Expression::Literal(s)}
            / s:bool_r() {Expression::Literal(s)}

        rule vcl_version() -> TopLevelDeclaration<'a>
            = vcl:vcl() number:number() semi:semicolon() {
                TopLevelDeclaration::VclVersion { vcl, number, semi }
            }

        rule include_decl() -> IncludeData<'a>
            = include:include() name:string() semi:semicolon() {
                IncludeData { include, name, semi }
            }

        rule import_from() -> FromData<'a>
            = from:from() value:string() {
                FromData { from, value }
            }

        rule import_decl() -> TopLevelDeclaration<'a>
            = import:import() name:ident() from:import_from()? semi:semicolon() {
                TopLevelDeclaration::Import { import, name, from, semi }
            }

        rule mask() -> MaskData<'a>
            = op:divide() mask:number() {
                MaskData { op, mask }
            }

        rule acl_entry() -> AclEntry<'a>
            = value:string() mask:mask()? semi:semicolon() {
                AclEntry { value, mask, semi }
            }

        rule acl_decl() -> TopLevelDeclaration<'a>
            = acl:acl() name:ident() lbrace:lBrace() entries:acl_entry()* rbrace:rBrace() {
                TopLevelDeclaration::Acl { acl, name, lbrace, entries, rbrace }
            }

        rule expression() -> Expression<'a> = precedence!{
            x:(@) op:plus() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            x:(@) op:minus() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            --
            x:(@) op:multiply() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            x:(@) op:divide() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            --
            x:(@) op:and() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            --
            x:(@) op:or() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            --
            x:(@) op:equals() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            x:(@) op:notEquals() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            x:(@) op:matches() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            x:(@) op:notMatches() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            x:(@) op:greater() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            x:(@) op:lesser() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            x:(@) op:greaterEquals() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            x:(@) op:lesserEquals() y:@ {Expression::Binary { left: Box::new(x), op, right: Box::new(y) }}
            --
            op:negate() x:@ {
                Expression::Neg { op, expr: Box::new(x) }
            }
            --
            l:literal() {l}
            e:ident_call_expr() {Expression::IdentCall(e)}
            i:ident() {Expression::Ident(i)}
            lparen:lParen() e:expression() rparen:rParen() {
                Expression::Parenthesized { lparen, expr: Box::new(e), rparen }
            }
        }

        rule function_call_arg() -> FunctionCallArg<'a>
            = name:ident() op:assign() value:expression() {
                FunctionCallArg::Named { name, op, value }
            }
            / e:expression() {FunctionCallArg::Positional(e)}

        rule ident_call_expr() -> IdentCallExpression<'a>
            = name:ident() lparen:lParen() args:list(<function_call_arg()>, <comma()>) rparen:rParen() {
                IdentCallExpression { name, lparen, args: Box::new(args), rparen }
            }

        rule string_list() -> Vec<TokenData<'a>>
            = string()*<2,>

        rule backend_value() -> BackendValue<'a>
            = strings:string_list() semi:semicolon() {
                BackendValue::StringList { strings, semi }
            }
            / expr:expression() semi:semicolon() {
                BackendValue::Expression { expr, semi }
            }
            / lbrace:lBrace() properties:backend_property()* rbrace:rBrace() {
                BackendValue::Composite { lbrace, properties, rbrace }
            }

        rule backend_property() -> BackendProperty<'a>
            = name:backendPropIdent() op:assign() value:backend_value() {
                BackendProperty { name, op, value }
            }

        rule backend_decl() -> TopLevelDeclaration<'a>
            = backend:backend() name:ident() lbrace:lBrace() properties:backend_property()* rbrace:rBrace() {
                TopLevelDeclaration::Backend(
                    BackendData::Defined { backend, name, lbrace, properties, rbrace }
                )
            }
            / backend:backend() name:ident() none:none() semi:semicolon() {
                TopLevelDeclaration::Backend(
                    BackendData::None { backend, name, none, semi }
                )
            }

        rule probe_decl() -> TopLevelDeclaration<'a>
            = probe:probe() name:ident() lbrace:lBrace() properties:backend_property()* rbrace:rBrace() {
                TopLevelDeclaration::Probe { probe, name, lbrace, properties, rbrace }
            }

        rule unset_statement() -> Statement<'a>
            = unset:unset() ident:ident() semi:semicolon() {
                Statement::Unset { unset, ident, semi }
            }

        rule set_statement() -> Statement<'a>
            = set:set() ident:ident() op:assign_op() expr:expression() semi:semicolon() {
                Statement::Set { set, ident, op, expr, semi }
            }

        rule if_statement() -> Statement<'a>
            = if_t:if_r() lparen:lParen() condition:expression() rparen:rParen() lbrace:lBrace() body:statement()* rbrace:rBrace() elseifs:elseif_statement()* else_st:else_statement()? {
                Statement::If { if_t, lparen, condition, rparen, lbrace, body, rbrace, elseifs, else_st }
            }

        rule elseif_keyword() -> Vec<TokenData<'a>>
            = else_t:else_r() if_t:if_r() {vec![else_t, if_t]}
            / elseif:elseIf() {vec![elseif]}

        rule elseif_statement() -> ElseIfStatement<'a>
            = elseif:elseif_keyword() lparen:lParen() condition:expression() rparen:rParen() lbrace:lBrace() body:statement()* rbrace:rBrace() {
                ElseIfStatement { elseif, lparen, condition, rparen, lbrace, body, rbrace }
            }

        rule else_statement() -> ElseStatement<'a>
            = else_t:else_r() lbrace:lBrace() body:statement()* rbrace:rBrace() {
                ElseStatement { else_t, lbrace, body, rbrace }
            }

        rule new_statement() -> Statement<'a>
            = new:new() name:ident() op:assign() value:ident_call_expr() semi:semicolon() {
                Statement::New { new, name, op, value, semi }
            }

        rule call_statement() -> Statement<'a>
            = call:call() ident:ident() semi:semicolon() {
                Statement::Call { call, ident, semi }
            }

        rule return_args() -> ReturnArgs<'a>
            = lparen:lParen() args:list(<expression()>, <comma()>) rparen:rParen() {
                ReturnArgs { lparen, args, rparen }
            }

        rule return_statement() -> Statement<'a>
            = return_t:return_r() lparen:lParen() name:ident() args:return_args()? rparen:rParen() semi:semicolon() {
                Statement::Return { return_t, lparen, name, args, rparen, semi }
            }

        rule statement() -> Statement<'a>
            = unset_statement()
            / set_statement()
            / if_statement()
            / new_statement()
            / call_statement()
            / expr:ident_call_expr() semi:semicolon() {
                Statement::IdentCall { expr, semi }
            }
            / i:include_decl() {Statement::Include(i)}
            / return_statement()

        rule sub_decl() -> TopLevelDeclaration<'a>
            = sub:sub() name:ident() lbrace:lBrace() statements:statement()* rbrace:rBrace() {
                TopLevelDeclaration::Sub { sub, name, lbrace, statements, rbrace }
            }

        rule top_level_declaration() -> TopLevelDeclaration<'a>
            = vcl_version()
            / i:include_decl() {TopLevelDeclaration::Include(i)}
            / import_decl()
            / acl_decl()
            / backend_decl()
            / probe_decl()
            / sub_decl()

        pub rule source_file() -> SourceFile<'a>
            = declarations:top_level_declaration()* eof:eof() {
                SourceFile { declarations, eof }
            }
    }
}
