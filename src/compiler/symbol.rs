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

pub const BOOL: [Terminal; 1] = [Terminal::Bool];

pub const INT: [Terminal; 8] = [
    Terminal::U8,
    Terminal::U16,
    Terminal::U32,
    Terminal::U64,
    Terminal::I8,
    Terminal::I16,
    Terminal::I32,
    Terminal::I64,
];

pub const FLOAT: [Terminal; 3] = [Terminal::F16, Terminal::F32, Terminal::F64];

pub const NUM: [Terminal; 11] = [
    Terminal::U8,
    Terminal::U16,
    Terminal::U32,
    Terminal::U64,
    Terminal::I8,
    Terminal::I16,
    Terminal::I32,
    Terminal::I64,
    Terminal::F16,
    Terminal::F32,
    Terminal::F64,
];

pub const ANY: [Terminal; 12] = [
    Terminal::Bool,
    Terminal::U8,
    Terminal::U16,
    Terminal::U32,
    Terminal::U64,
    Terminal::I8,
    Terminal::I16,
    Terminal::I32,
    Terminal::I64,
    Terminal::F16,
    Terminal::F32,
    Terminal::F64,
];
