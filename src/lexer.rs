use logos::{Lexer, Logos, Skip};

pub fn lex<'a>(data_str: &'a str) -> Result<Vec<Token<'a>>, ()> {
    let mut lex = Token::lexer(data_str);
    let mut iter = TokenIter {
        lex,
        lex_done: false,
    };
    iter.collect()
}

pub fn lex_trivia<'a>(data_str: &'a str) -> Result<Vec<TriviaToken<'a>>, ()> {
    let mut lex = TriviaToken::lexer(data_str);
    lex.collect()
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TokenData<'a> {
    pub content: &'a str,
    pub line: usize,
    pub column: usize,
    pub pre_trivia: &'a str,
}

#[derive(Default)]
pub struct LexerState {
    line: usize,
    last_line_end: usize,
    last_token_end: usize,
}

struct TokenIter<'a> {
    lex: Lexer<'a, Token<'a>>,
    lex_done: bool,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Result<Token<'a>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lex_done {
            None
        } else if let Some(r) = self.lex.next() {
            Some(r)
        } else {
            self.lex_done = true;
            let final_trivia = &self.lex.source()[self.lex.extras.last_token_end..];
            let data = TokenData {
                content: "",
                line: self.lex.extras.line + 1,
                column: self.lex.source().len() - self.lex.extras.last_line_end + 1,
                pre_trivia: final_trivia,
            };
            Some(Ok(Token::EOF(data)))
        }
    }
}

fn newline_callback<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Skip {
    lex.extras.line += 1;
    lex.extras.last_line_end = lex.span().end;
    Skip
}

fn comment<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Skip {
    lex.extras.line += lex.slice().chars().filter(|c| *c == '\n').count();
    Skip
}

fn token_callback<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<TokenData<'a>> {
    let start = lex.span().start;
    let column = start - lex.extras.last_line_end + 1;
    let pre_trivia = &lex.source()[lex.extras.last_token_end..start];
    lex.extras.last_token_end = lex.span().end;
    Some(TokenData {
        content: lex.slice(),
        line: lex.extras.line + 1,
        column,
        pre_trivia,
    })
}

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(skip r"[ \t]+", extras = LexerState)]
pub enum Token<'a> {
    #[token("acl", token_callback)]
    Acl(TokenData<'a>),

    #[token("vcl", token_callback)]
    Vcl(TokenData<'a>),

    #[token("import", token_callback)]
    Import(TokenData<'a>),

    #[token("include", token_callback)]
    Include(TokenData<'a>),

    #[token("from", token_callback)]
    From(TokenData<'a>),

    #[token("probe", token_callback)]
    Probe(TokenData<'a>),

    #[token("backend", token_callback)]
    Backend(TokenData<'a>),

    #[token("none", token_callback)]
    None(TokenData<'a>),

    #[token("sub", token_callback)]
    Sub(TokenData<'a>),

    #[token("set", token_callback)]
    Set(TokenData<'a>),

    #[token("call", token_callback)]
    Call(TokenData<'a>),

    #[token("unset", token_callback)]
    Unset(TokenData<'a>),

    #[token("if", token_callback)]
    If(TokenData<'a>),

    #[token("else", token_callback)]
    Else(TokenData<'a>),

    #[token("elseif", token_callback)]
    ElseIf(TokenData<'a>),

    #[token("return", token_callback)]
    Return(TokenData<'a>),

    #[token("new", token_callback)]
    New(TokenData<'a>),

    #[token("true", token_callback)]
    #[token("false", token_callback)]
    Bool(TokenData<'a>),

    #[regex(r"-?(0|[1-9]\d*)(\.\d+)?", token_callback)]
    Number(TokenData<'a>),

    #[regex(r"-?(0|[1-9]\d*)(\.\d+)?(ms|s|m|h|d|w|y)", token_callback)]
    Duration(TokenData<'a>),

    #[regex(r"-?(0|[1-9]\d*)(\.\d+)?(B|KB|MB|GB|TB)", token_callback)]
    Bytes(TokenData<'a>),

    #[regex(r#""[^"\r\n]*""#, token_callback)]
    #[regex(r#""""([^"]|"[^"]|""[^"])*""""#, token_callback)]
    #[regex(r#"\{"([^"]|"[^\}])*"\}"#, token_callback)]
    String(TokenData<'a>),

    #[regex(r"[a-zA-Z_][\w\-]*(\.[a-zA-Z_][\w\-]*)*", token_callback)]
    Ident(TokenData<'a>),

    #[regex(r"\.[a-zA-Z_][\w\-]*", token_callback)]
    BackendPropIdent(TokenData<'a>),

    #[token(";", token_callback)]
    Semicolon(TokenData<'a>),

    #[token("{", token_callback)]
    LBrace(TokenData<'a>),

    #[token("}", token_callback)]
    RBrace(TokenData<'a>),

    #[token("(", token_callback)]
    LParen(TokenData<'a>),

    #[token(")", token_callback)]
    RParen(TokenData<'a>),

    #[token("!", token_callback)]
    Negate(TokenData<'a>),

    #[token("=", token_callback)]
    Assign(TokenData<'a>),

    #[token("+", token_callback)]
    Plus(TokenData<'a>),

    #[token("-", token_callback)]
    Minus(TokenData<'a>),

    #[token("*", token_callback)]
    Multiply(TokenData<'a>),

    #[token("/", token_callback)]
    Divide(TokenData<'a>),

    #[token(",", token_callback)]
    Comma(TokenData<'a>),

    #[token("||", token_callback)]
    Or(TokenData<'a>),

    #[token("&&", token_callback)]
    And(TokenData<'a>),

    #[token("==", token_callback)]
    Equals(TokenData<'a>),

    #[token("!=", token_callback)]
    NotEquals(TokenData<'a>),

    #[token("~", token_callback)]
    Matches(TokenData<'a>),

    #[token(">", token_callback)]
    Greater(TokenData<'a>),

    #[token("<", token_callback)]
    Lesser(TokenData<'a>),

    #[token(">=", token_callback)]
    GreaterEquals(TokenData<'a>),

    #[token("<=", token_callback)]
    LesserEquals(TokenData<'a>),

    #[token("++", token_callback)]
    Increment(TokenData<'a>),

    #[token("--", token_callback)]
    Decrement(TokenData<'a>),

    #[token("<<", token_callback)]
    ShiftLeft(TokenData<'a>),

    #[token(">>", token_callback)]
    ShiftRight(TokenData<'a>),

    #[token("+=", token_callback)]
    AddAssign(TokenData<'a>),

    #[token("-=", token_callback)]
    SubtractAssign(TokenData<'a>),

    #[token("*=", token_callback)]
    MultiplyAssign(TokenData<'a>),

    #[token("/=", token_callback)]
    DivideAssign(TokenData<'a>),

    #[token("!~", token_callback)]
    NotMatches(TokenData<'a>),

    #[token("%", token_callback)]
    Modulo(TokenData<'a>),

    #[token("&", token_callback)]
    BitwiseAnd(TokenData<'a>),

    #[token("|", token_callback)]
    BitwiseOr(TokenData<'a>),

    #[regex(r"(//|#).*", comment)]
    LineComment,

    #[regex(r"/\*([^*]|\*[^/])*\*/", comment)]
    MultilineComment,

    #[regex(r#"C\{([^\}]|\}[^C])*\}C"#, comment)]
    InlineCCode,

    #[regex(r"(\r\n|\n|\r)", newline_callback)]
    Newline,

    EOF(TokenData<'a>),
}

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(skip r"[ \t]+")]
pub enum TriviaToken<'a> {
    #[regex(r"(//|#).*")]
    LineComment(&'a str),

    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    MultilineComment(&'a str),

    #[regex(r#"C\{([^\}]|\}[^C])*\}C"#)]
    InlineCCode(&'a str),

    #[regex(r"(\r\n|\n|\r)")]
    Newline,
}

pub fn get_token_data<'a>(t: Token<'a>) -> Option<TokenData<'a>> {
    match t {
        Token::Acl(x) => Some(x),
        Token::Vcl(x) => Some(x),
        Token::Import(x) => Some(x),
        Token::Include(x) => Some(x),
        Token::From(x) => Some(x),
        Token::Probe(x) => Some(x),
        Token::Backend(x) => Some(x),
        Token::None(x) => Some(x),
        Token::Sub(x) => Some(x),
        Token::Set(x) => Some(x),
        Token::Call(x) => Some(x),
        Token::Unset(x) => Some(x),
        Token::If(x) => Some(x),
        Token::Else(x) => Some(x),
        Token::ElseIf(x) => Some(x),
        Token::Return(x) => Some(x),
        Token::New(x) => Some(x),
        Token::Bool(x) => Some(x),
        Token::Number(x) => Some(x),
        Token::Duration(x) => Some(x),
        Token::Bytes(x) => Some(x),
        Token::String(x) => Some(x),
        Token::Ident(x) => Some(x),
        Token::BackendPropIdent(x) => Some(x),
        Token::Semicolon(x) => Some(x),
        Token::LBrace(x) => Some(x),
        Token::RBrace(x) => Some(x),
        Token::LParen(x) => Some(x),
        Token::RParen(x) => Some(x),
        Token::Negate(x) => Some(x),
        Token::Assign(x) => Some(x),
        Token::Plus(x) => Some(x),
        Token::Minus(x) => Some(x),
        Token::Multiply(x) => Some(x),
        Token::Divide(x) => Some(x),
        Token::Comma(x) => Some(x),
        Token::Or(x) => Some(x),
        Token::And(x) => Some(x),
        Token::Equals(x) => Some(x),
        Token::NotEquals(x) => Some(x),
        Token::Matches(x) => Some(x),
        Token::Greater(x) => Some(x),
        Token::Lesser(x) => Some(x),
        Token::GreaterEquals(x) => Some(x),
        Token::LesserEquals(x) => Some(x),
        Token::Increment(x) => Some(x),
        Token::Decrement(x) => Some(x),
        Token::ShiftLeft(x) => Some(x),
        Token::ShiftRight(x) => Some(x),
        Token::AddAssign(x) => Some(x),
        Token::SubtractAssign(x) => Some(x),
        Token::MultiplyAssign(x) => Some(x),
        Token::DivideAssign(x) => Some(x),
        Token::NotMatches(x) => Some(x),
        Token::Modulo(x) => Some(x),
        Token::BitwiseAnd(x) => Some(x),
        Token::BitwiseOr(x) => Some(x),
        Token::LineComment => None,
        Token::MultilineComment => None,
        Token::InlineCCode => None,
        Token::Newline => None,
        Token::EOF(x) => Some(x),
    }
}
