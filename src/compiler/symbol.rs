#[derive(Debug, PartialEq)]
pub enum Symbol {
    Type(Type),
    Var(Type),
    Literal(Terminal),
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Func(Func),
    Terminal(Terminal),
}

#[derive(Debug, PartialEq)]
pub struct Func {
    pub params: Vec<Terminal>,
    pub ret: Terminal,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Terminal {
    Void,
    Bool,
    I32,
    F32,
}
