use crate::lexer::TokenData;

pub struct SourceFile<'a> {
    pub declarations: Vec<TopLevelDeclaration<'a>>,
    pub eof: TokenData<'a>,
}

#[derive(Debug)]
pub enum TopLevelDeclaration<'a> {
    VclVersion {
        vcl: TokenData<'a>,
        number: TokenData<'a>,
        semi: TokenData<'a>,
    },
    Import {
        import: TokenData<'a>,
        name: TokenData<'a>,
        from: Option<FromData<'a>>,
        semi: TokenData<'a>,
    },
    Include(IncludeData<'a>),
    Acl {
        acl: TokenData<'a>,
        name: TokenData<'a>,
        lbrace: TokenData<'a>,
        entries: Vec<AclEntry<'a>>,
        rbrace: TokenData<'a>,
    },
    Backend(BackendData<'a>),
    Probe {
        probe: TokenData<'a>,
        name: TokenData<'a>,
        lbrace: TokenData<'a>,
        properties: Vec<BackendProperty<'a>>,
        rbrace: TokenData<'a>,
    },
    Sub {
        sub: TokenData<'a>,
        name: TokenData<'a>,
        lbrace: TokenData<'a>,
        statements: Vec<Statement<'a>>,
        rbrace: TokenData<'a>,
    },
}

#[derive(Debug)]
pub enum BackendData<'a> {
    Defined {
        backend: TokenData<'a>,
        name: TokenData<'a>,
        lbrace: TokenData<'a>,
        properties: Vec<BackendProperty<'a>>,
        rbrace: TokenData<'a>,
    },
    None {
        backend: TokenData<'a>,
        name: TokenData<'a>,
        none: TokenData<'a>,
        semi: TokenData<'a>,
    },
}

#[derive(Debug)]
pub struct IncludeData<'a> {
    pub include: TokenData<'a>,
    pub name: TokenData<'a>,
    pub semi: TokenData<'a>,
}

#[derive(Debug)]
pub struct FromData<'a> {
    pub from: TokenData<'a>,
    pub value: TokenData<'a>,
}

#[derive(Debug)]
pub struct AclEntry<'a> {
    pub value: TokenData<'a>,
    pub mask: Option<MaskData<'a>>,
    pub semi: TokenData<'a>,
}

#[derive(Debug)]
pub struct MaskData<'a> {
    pub op: TokenData<'a>,
    pub mask: TokenData<'a>,
}

#[derive(Debug)]
pub struct BackendProperty<'a> {
    pub name: TokenData<'a>,
    pub op: TokenData<'a>,
    pub value: BackendValue<'a>,
}

#[derive(Debug)]
pub enum BackendValue<'a> {
    Expression {
        expr: Expression<'a>,
        semi: TokenData<'a>,
    },
    StringList {
        strings: Vec<TokenData<'a>>,
        semi: TokenData<'a>,
    },
    Composite {
        lbrace: TokenData<'a>,
        properties: Vec<BackendProperty<'a>>,
        rbrace: TokenData<'a>,
    },
}

#[derive(Debug)]
pub struct ElseIfStatement<'a> {
    pub elseif: Vec<TokenData<'a>>,
    pub lparen: TokenData<'a>,
    pub condition: Expression<'a>,
    pub rparen: TokenData<'a>,
    pub lbrace: TokenData<'a>,
    pub body: Vec<Statement<'a>>,
    pub rbrace: TokenData<'a>,
}

#[derive(Debug)]
pub struct ElseStatement<'a> {
    pub else_t: TokenData<'a>,
    pub lbrace: TokenData<'a>,
    pub body: Vec<Statement<'a>>,
    pub rbrace: TokenData<'a>,
}

#[derive(Debug)]
pub struct IdentCallExpression<'a> {
    pub name: TokenData<'a>,
    pub lparen: TokenData<'a>,
    pub args: Box<DelimitedList<FunctionCallArg<'a>, TokenData<'a>>>,
    pub rparen: TokenData<'a>,
}

#[derive(Debug)]
pub struct ReturnArgs<'a> {
    pub lparen: TokenData<'a>,
    pub args: DelimitedList<Expression<'a>, TokenData<'a>>,
    pub rparen: TokenData<'a>,
}

#[derive(Debug)]
pub enum Statement<'a> {
    Set {
        set: TokenData<'a>,
        ident: TokenData<'a>,
        op: TokenData<'a>,
        expr: Expression<'a>,
        semi: TokenData<'a>,
    },
    Unset {
        unset: TokenData<'a>,
        ident: TokenData<'a>,
        semi: TokenData<'a>,
    },
    Call {
        call: TokenData<'a>,
        ident: TokenData<'a>,
        semi: TokenData<'a>,
    },
    IdentCall {
        expr: IdentCallExpression<'a>,
        semi: TokenData<'a>,
    },
    If {
        if_t: TokenData<'a>,
        lparen: TokenData<'a>,
        condition: Expression<'a>,
        rparen: TokenData<'a>,
        lbrace: TokenData<'a>,
        body: Vec<Statement<'a>>,
        rbrace: TokenData<'a>,
        elseifs: Vec<ElseIfStatement<'a>>,
        else_st: Option<ElseStatement<'a>>,
    },
    Return {
        return_t: TokenData<'a>,
        lparen: TokenData<'a>,
        name: TokenData<'a>,
        args: Option<ReturnArgs<'a>>,
        rparen: TokenData<'a>,
        semi: TokenData<'a>,
    },
    New {
        new: TokenData<'a>,
        name: TokenData<'a>,
        op: TokenData<'a>,
        value: IdentCallExpression<'a>,
        semi: TokenData<'a>,
    },
    Include(IncludeData<'a>),
}

#[derive(Debug)]
pub enum Expression<'a> {
    Ident(TokenData<'a>),
    Literal(TokenData<'a>),
    Neg {
        op: TokenData<'a>,
        expr: Box<Expression<'a>>,
    },
    Binary {
        left: Box<Expression<'a>>,
        op: TokenData<'a>,
        right: Box<Expression<'a>>,
    },
    IdentCall(IdentCallExpression<'a>),
    Parenthesized {
        lparen: TokenData<'a>,
        expr: Box<Expression<'a>>,
        rparen: TokenData<'a>,
    },
}

#[derive(Debug)]
pub enum FunctionCallArg<'a> {
    Named {
        name: TokenData<'a>,
        op: TokenData<'a>,
        value: Expression<'a>,
    },
    Positional(Expression<'a>),
}

#[derive(Debug)]
pub enum DelimitedList<Item, Separator> {
    Empty,
    WithItems {
        pairs: Vec<(Item, Separator)>,
        last_item: Item,
    },
}

impl<Item, Separator> DelimitedList<Item, Separator> {
    pub fn iter(&self) -> DelimitedListIterator<'_, Item, Separator> {
        DelimitedListIterator {
            list: self,
            inner_iter: None,
            done: false,
        }
    }
}

pub struct DelimitedListIterator<'a, Item, Separator> {
    list: &'a DelimitedList<Item, Separator>,
    inner_iter: Option<std::slice::Iter<'a, (Item, Separator)>>,
    done: bool,
}

impl<'a, Item, Separator> Iterator for DelimitedListIterator<'a, Item, Separator> {
    type Item = (&'a Item, Option<&'a Separator>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        match self.list {
            DelimitedList::Empty => {
                self.done = true;
                None
            }
            DelimitedList::WithItems { pairs, last_item } => {
                let inner_iter = {
                    match &mut self.inner_iter {
                        Some(i) => i,
                        None => {
                            self.inner_iter = Some(pairs.iter());
                            match &mut self.inner_iter {
                                Some(i) => i,
                                None => panic!("should never happen, just set it to Some"),
                            }
                        }
                    }
                };
                let inner_next = inner_iter.next();
                match inner_next {
                    Some(i) => Some((&i.0, Some(&i.1))),
                    None => {
                        self.done = true;
                        Some((last_item, None))
                    }
                }
            }
        }
    }
}
