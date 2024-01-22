use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(skip r"[ \t]+")]
pub enum Token<'a> {
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

    #[token("elseif")]
    ElseIf,

    #[token("return")]
    Return,

    #[token("new")]
    New,

    #[token("true")]
    #[token("false")]
    Bool(&'a str),

    #[regex(r"-?(0|[1-9]\d*)(\.\d+)?")]
    Number(&'a str),

    #[regex(r"-?(0|[1-9]\d*)(\.\d+)?(ms|s|m|h|d|w|y)")]
    Duration(&'a str),

    #[regex(r"-?(0|[1-9]\d*)(\.\d+)?(B|KB|MB|GB|TB)")]
    Bytes(&'a str),

    #[regex(r#""[^"\r\n]*""#)]
    #[regex(r#""""([^"]|"[^"]|""[^"])*""""#)]
    #[regex(r#"\{"([^"]|"[^\}])*"\}"#)]
    String(&'a str),

    #[regex(r"[a-zA-Z_][\w\-]*(\.[a-zA-Z_][\w\-]*)*")]
    Ident(&'a str),

    #[regex(r"\.[a-zA-Z_][\w\-]*")]
    BackendPropIdent(&'a str),

    #[token(";")]
    Semicolon,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

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
    LineComment(&'a str),

    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    MultilineComment(&'a str),

    #[regex(r#"C\{([^\}]|\}[^C])*\}C"#)]
    InlineCCode(&'a str),

    // maybe?
    #[regex(r"(\r\n|\n|\r)")]
    Newline(&'a str),
}
