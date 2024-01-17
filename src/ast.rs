pub type SourceFile<'a> = Vec<TopLevelDeclaration<'a>>;

pub enum TopLevelDeclaration<'a> {
    VclVersion(&'a str),
    Import {
        name: &'a str,
        from: Option<&'a str>,
    },
    Include(&'a str),
    Acl {
        name: &'a str,
        entries: Vec<AclEntry<'a>>,
    },
    Backend {
        name: &'a str,
        properties: Vec<BackendProperty<'a>>,
    },
    Probe {
        name: &'a str,
        properties: Vec<BackendProperty<'a>>,
    },
    Sub {
        name: &'a str,
        statements: Vec<Statement<'a>>,
    },
}

pub struct AclEntry<'a> {
    pub value: &'a str,
    pub mask: Option<u8>,
}

pub struct BackendProperty<'a> {
    pub name: &'a str,
    pub values: Vec<BackendValue<'a>>,
}

pub enum BackendValue<'a> {
    Expression(Expression<'a>),
    StringList(Vec<&'a str>),
    Composite(Vec<BackendProperty<'a>>),
}

pub enum Statement<'a> {
    Set {
        ident: &'a str,
        op: AssignOperator,
        expr: Expression<'a>,
    },
    Unset {
        ident: &'a str,
    },
    Call {
        ident: &'a str,
    },
    IdentCall, // TODO:
    If,        // TODO:
    Return,    // TODO:
    New,       // TODO:
}

pub enum AssignOperator {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

// TODO:
pub enum Expression<'a> {
    Ident(&'a str),
    // Number(&'a str),
    // String(&'a str),
    // BinaryOp,
    // UnaryOp,
    // Call,
    // IdentCall,
}
