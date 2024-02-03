use crate::lexer::Token;

type WS<'a> = Vec<Token<'a>>;

pub type SourceFile<'a> = Vec<TopLevelDeclaration<'a>>;

#[derive(Debug)]
pub enum TopLevelDeclaration<'a> {
    VclVersion {
        ws_pre_vcl: WS<'a>,
        ws_pre_number: WS<'a>,
        ws_pre_semi: WS<'a>,
        number: &'a str,
    },
    Import {
        ws_pre_import: WS<'a>,
        ws_pre_name: WS<'a>,
        ws_pre_semi: WS<'a>,
        name: &'a str,
        from: Option<FromData<'a>>,
    },
    Include(IncludeData<'a>),
    Acl {
        ws_pre_acl: WS<'a>,
        ws_pre_name: WS<'a>,
        ws_pre_lbrace: WS<'a>,
        ws_pre_rbrace: WS<'a>,
        name: &'a str,
        entries: Vec<AclEntry<'a>>,
    },
    Backend(BackendData<'a>),
    Probe {
        ws_pre_probe: WS<'a>,
        ws_pre_name: WS<'a>,
        ws_pre_lbrace: WS<'a>,
        ws_pre_rbrace: WS<'a>,
        name: &'a str,
        properties: Vec<BackendProperty<'a>>,
    },
    Sub {
        name: &'a str,
        statements: Vec<Statement<'a>>,
    },
}

#[derive(Debug)]
pub enum BackendData<'a> {
    Defined {
        ws_pre_backend: WS<'a>,
        ws_pre_name: WS<'a>,
        ws_pre_lbrace: WS<'a>,
        ws_pre_rbrace: WS<'a>,
        name: &'a str,
        properties: Vec<BackendProperty<'a>>,
    },
    None {
        ws_pre_backend: WS<'a>,
        ws_pre_name: WS<'a>,
        ws_pre_none: WS<'a>,
        ws_pre_semi: WS<'a>,
        name: &'a str,
    },
}

#[derive(Debug)]
pub struct IncludeData<'a> {
    pub ws_pre_include: WS<'a>,
    pub ws_pre_name: WS<'a>,
    pub ws_pre_semi: WS<'a>,
    pub name: &'a str,
}

#[derive(Debug)]
pub struct FromData<'a> {
    pub ws_pre_from: WS<'a>,
    pub ws_pre_value: WS<'a>,
    pub value: &'a str,
}

#[derive(Debug)]
pub struct AclEntry<'a> {
    pub ws_pre_value: WS<'a>,
    pub ws_pre_semi: WS<'a>,
    pub value: &'a str,
    pub mask: Option<MaskData<'a>>,
}

#[derive(Debug)]
pub struct MaskData<'a> {
    pub ws_pre_op: WS<'a>,
    pub ws_pre_mask: WS<'a>,
    pub mask: &'a str,
}

#[derive(Debug)]
pub struct BackendProperty<'a> {
    pub ws_pre_name: WS<'a>,
    pub ws_pre_op: WS<'a>,
    pub ws_pre_semi: WS<'a>,
    pub name: &'a str,
    pub value: BackendValue<'a>,
}

#[derive(Debug)]
pub enum BackendValue<'a> {
    Expression(Expression<'a>),
    StringList(Vec<StringListEntry<'a>>),
    Composite {
        ws_pre_lbrace: WS<'a>,
        ws_pre_rbrace: WS<'a>,
        properties: Vec<BackendProperty<'a>>
    },
}

#[derive(Debug)]
pub struct StringListEntry<'a> {
    pub ws_pre_string: WS<'a>,
    pub string: &'a str,
}

#[derive(Debug)]
pub struct ElseIfStatement<'a> {
    pub condition: Expression<'a>,
    pub body: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub struct IdentCallExpression<'a> {
    pub name: &'a str,
    pub args: Vec<FunctionCallArg<'a>>,
}

#[derive(Debug)]
pub enum Statement<'a> {
    Set {
        ident: &'a str,
        op: &'a str,
        expr: Expression<'a>,
    },
    Unset {
        ident: &'a str,
    },
    Call {
        ident: &'a str,
    },
    IdentCall(IdentCallExpression<'a>),
    If {
        condition: Expression<'a>,
        body: Vec<Statement<'a>>,
        elseifs: Vec<ElseIfStatement<'a>>,
        else_st: Option<Vec<Statement<'a>>>,
    },
    Return {
        name: &'a str,
        args: Option<Vec<Expression<'a>>>,
    },
    New {
        name: &'a str,
        value: IdentCallExpression<'a>,
    },
    Include(IncludeData<'a>),
}

#[derive(Debug)]
pub enum Expression<'a> {
    Ident(&'a str),
    Literal(&'a str),
    Neg(Box<Expression<'a>>),
    Binary {
        left: Box<Expression<'a>>,
        op: &'a str,
        right: Box<Expression<'a>>,
    },
    IdentCall(IdentCallExpression<'a>),
}

#[derive(Debug)]
pub enum FunctionCallArg<'a> {
    Named {
        name: &'a str,
        value: Expression<'a>,
    },
    Positional(Expression<'a>),
}
