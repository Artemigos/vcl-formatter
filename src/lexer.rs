use logos::{Lexer, Logos, Skip};

pub fn lex(data_str: &str) -> Result<Vec<Token<'_>>, crate::error::E> {
    let lex = Token::lexer(data_str);
    let iter = TokenIter {
        lex,
        lex_done: false,
    };
    iter.collect()
}

pub fn lex_trivia(data_str: &str) -> Result<Vec<TriviaToken<'_>>, crate::error::E> {
    let lex =
        TriviaToken::lexer(data_str).map(|x| x.map_err(|_| crate::error::E::LexingTriviaFailed));
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
    type Item = Result<Token<'a>, crate::error::E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lex_done {
            None
        } else if let Some(r) = self.lex.next() {
            match r {
                Ok(t) => Some(Ok(t)),
                Err(_) => {
                    let (line, column) = position(&self.lex, self.lex.span().start);
                    Some(Err(crate::error::E::LexingFailed { line, column }))
                }
            }
        } else {
            self.lex_done = true;
            let final_trivia = &self.lex.source()[self.lex.extras.last_token_end..];
            let (line, column) = position(&self.lex, self.lex.source().len());
            let data = TokenData {
                content: "",
                line,
                column,
                pre_trivia: final_trivia,
            };
            Some(Ok(Token::Eof(data)))
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
    let (line, column) = position(lex, start);
    let pre_trivia = &lex.source()[lex.extras.last_token_end..start];
    lex.extras.last_token_end = lex.span().end;
    Some(TokenData {
        content: lex.slice(),
        line,
        column,
        pre_trivia,
    })
}

fn position<'a>(lex: &Lexer<'a, Token<'a>>, start: usize) -> (usize, usize) {
    let column = start - lex.extras.last_line_end + 1;
    let line = lex.extras.line + 1;
    (line, column)
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

    Eof(TokenData<'a>),
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
