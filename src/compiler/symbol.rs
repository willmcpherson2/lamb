#[derive(Debug, PartialEq)]
pub enum Symbol {
    Module,
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
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F16,
    F32,
    F64,
}
