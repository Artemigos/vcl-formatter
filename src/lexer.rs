use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t]+")]
pub enum Token {
    #[token("acl")]
    Acl,

    #[token("vcl")]
    Vcl,

    #[token("import")]
    Import,

    #[token("include")]
    Include,

    #[token("from")]
    From,

    #[token("probe")]
    Probe,

    #[token("backend")]
    Backend,

    #[token("none")]
    None,

    #[token("sub")]
    Sub,

    #[token("set")]
    Set,

    #[token("call")]
    Call,

    #[token("unset")]
    Unset,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("return")]
    Return,

    #[token("new")]
    New,

    #[regex(r"-?(0|[1-9]\d*)(\.\d+)?")]
    Number,

    #[token(";")]
    Semicolon,

    #[regex(r#""[^"\r\n]*""#)]
    #[regex(r#""""([^"]|"[^"]|""[^"])*""""#)]
    #[regex(r#"\{"([^"]|"[^\}])*"\}"#)]
    String,

    #[regex(r"[a-zA-Z_][\w\-]*(\.[a-zA-Z_][\w\-]*)*")]
    Ident,

    #[token("{")]
    LBracket,

    #[token("}")]
    RBracket,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token(".")]
    Dot,

    #[token("!")]
    Negate,

    #[token("=")]
    Assign,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token(",")]
    Comma,

    #[token("||")]
    Or,

    #[token("&&")]
    And,

    #[token("==")]
    Equals,

    #[token("!=")]
    NotEquals,

    #[token("~")]
    Matches,

    #[token(">")]
    Greater,

    #[token("<")]
    Lesser,

    #[token(">=")]
    GreaterEquals,

    #[token("<=")]
    LesserEquals,

    #[token("++")]
    Increment,

    #[token("--")]
    Decrement,

    #[token("<<")]
    ShiftLeft,

    #[token(">>")]
    ShiftRight,

    #[token("+=")]
    AddAssign,

    #[token("-=")]
    SubtractAssign,

    #[token("*=")]
    MultiplyAssign,

    #[token("/=")]
    DivideAssign,

    #[token("!~")]
    NotMatches,

    #[token("%")]
    Modulo,

    #[token("&")]
    BitwiseAnd,

    #[token("|")]
    BitwiseOr,

    #[regex(r"(//|#).*")]
    LineComment,

    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    MultilineComment,

    #[regex(r#"C\{([^\}]|\}[^C])*\}C"#)]
    InlineCCode,

    // maybe?
    #[regex(r"(\r\n|\n|\r)")]
    Newline,
}
