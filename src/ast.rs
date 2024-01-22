pub type SourceFile<'a> = Vec<TopLevelDeclaration<'a>>;

#[derive(Debug)]
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
        properties: Option<Vec<BackendProperty<'a>>>,
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

#[derive(Debug)]
pub struct AclEntry<'a> {
    pub value: &'a str,
    pub mask: Option<&'a str>,
}

#[derive(Debug)]
pub struct BackendProperty<'a> {
    pub name: &'a str,
    pub values: Vec<BackendValue<'a>>,
}

#[derive(Debug)]
pub enum BackendValue<'a> {
    Expression(Expression<'a>),
    StringList(Vec<&'a str>),
    Composite(Vec<BackendProperty<'a>>),
}

#[derive(Debug)]
pub struct ElseIfStatement<'a> {
    pub condition: Expression<'a>,
    pub body: Vec<Statement<'a>>,
}

#[derive(Debug)]
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
    If {
        condition: Expression<'a>,
        body: Vec<Statement<'a>>,
        elseifs: Vec<ElseIfStatement<'a>>,
        else_st: Option<Vec<Statement<'a>>>,
    },
    Return, // TODO:
    New {
        name: &'a str,
        value: Expression<'a>,
    },
}

#[derive(Debug)]
pub enum AssignOperator {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

// TODO:
#[derive(Debug)]
pub enum Expression<'a> {
    Ident(&'a str),
    Literal(&'a str),
    Neg(Box<Expression<'a>>),
    // BinaryOp,
    IdentCall {
        name: &'a str,
        args: Vec<FunctionCallArg<'a>>,
    },
}

#[derive(Debug)]
pub enum FunctionCallArg<'a> {
    Named {
        name: &'a str,
        value: Expression<'a>,
    },
    Positional(Expression<'a>),
}
