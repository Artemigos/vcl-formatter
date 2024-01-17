use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
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

    #[regex(r"-?[0-9]+(\.[-0-9]+)?")]
    Number,

    #[token(";")]
    Semicolon,

    #[regex(r#""[^"]*""#)]
    String,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*")]
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
    Times,

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

    #[regex(r"(//|#).*")]
    LineComment,

    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    MultilineComment,
}
