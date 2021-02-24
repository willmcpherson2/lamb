use super::common::Id;
use super::symbol::Func;
use super::symbol::Symbol;
use super::symbol::Terminal;
use super::symbol::Type;
use crate::compiler::symbol::ANY;
use crate::compiler::symbol::BOOL;
use crate::compiler::symbol::INT;
use crate::compiler::symbol::NUM;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Namespace {
    symbol: Symbol,
    namespace: HashMap<String, Vec<Namespace>>,
}

impl From<(Symbol, HashMap<String, Vec<Namespace>>)> for Namespace {
    fn from(parts: (Symbol, HashMap<String, Vec<Self>>)) -> Self {
        Self {
            symbol: parts.0,
            namespace: parts.1,
        }
    }
}

impl From<Symbol> for Namespace {
    fn from(symbol: Symbol) -> Self {
        Self {
            symbol,
            namespace: HashMap::new(),
        }
    }
}

impl Namespace {
    pub fn new_module() -> Self {
        Self::from((Symbol::Module, builtins()))
    }

    pub const fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    pub fn get(&self, key: &str) -> Option<&Vec<Self>> {
        self.namespace.get(key)
    }

    pub fn get_or<'a>(&'a self, other: &'a Self, key: &str) -> Option<&'a Vec<Self>> {
        self.get(key).or_else(|| other.get(key))
    }

    pub fn get_then<'a>(&'a self, key: &str, id: Id) -> Option<&'a Self> {
        self.get(key).and_then(|symbols| symbols.get(id))
    }

    pub fn get_or_then<'a>(&'a self, other: &'a Self, key: &str, id: Id) -> Option<&'a Self> {
        self.get_or(other, key).and_then(|symbols| symbols.get(id))
    }

    pub fn insert_namespaces(&mut self, key: String, namespaces: Vec<Self>) {
        self.namespace.insert(key, namespaces);
    }

    pub fn append_namespace(&mut self, key: &str, namespace: Self) -> Id {
        if let Some(namespaces) = self.namespace.get_mut(key) {
            let id = namespaces.len();
            namespaces.push(namespace);
            id
        } else {
            self.namespace.insert(key.to_string(), vec![namespace]);
            0
        }
    }
}

fn builtins() -> HashMap<String, Vec<Namespace>> {
    macro_rules! typ {
        ($s:literal, $terminal:tt) => {
            (
                $s.to_string(),
                vec![Namespace::from(Symbol::Type(Type::Terminal(
                    Terminal::$terminal,
                )))],
            )
        };
    }

    macro_rules! unary {
        ($s:literal, $params:ident) => {{
            (
                $s.to_string(),
                $params
                    .iter()
                    .map(|param| {
                        Namespace::from(Symbol::Var(Type::Func(Func {
                            params: vec![*param],
                            ret: *param,
                        })))
                    })
                    .collect(),
            )
        }};
    }

    macro_rules! binary {
        ($s:literal, $params:ident) => {{
            (
                $s.to_string(),
                $params
                    .iter()
                    .map(|param| {
                        Namespace::from(Symbol::Var(Type::Func(Func {
                            params: vec![*param, *param],
                            ret: *param,
                        })))
                    })
                    .collect(),
            )
        }};

        ($s:literal, $params:ident, $ret:expr) => {{
            (
                $s.to_string(),
                $params
                    .iter()
                    .map(|param| {
                        Namespace::from(Symbol::Var(Type::Func(Func {
                            params: vec![*param, *param],
                            ret: $ret,
                        })))
                    })
                    .collect(),
            )
        }};
    }

    vec![
        typ!("void", Void),
        typ!("bool", Bool),
        typ!("u8", U8),
        typ!("u16", U16),
        typ!("u32", U32),
        typ!("u64", U64),
        typ!("i8", I8),
        typ!("i16", I16),
        typ!("i32", I32),
        typ!("i64", I64),
        typ!("f16", F16),
        typ!("f32", F32),
        typ!("f64", F64),
        unary!("!", BOOL),
        unary!("~", INT),
        binary!("+", NUM),
        binary!("-", NUM),
        binary!("*", NUM),
        binary!("/", NUM),
        binary!("%", NUM),
        binary!("&", INT),
        binary!("|", INT),
        binary!("^", INT),
        binary!("<<", INT),
        binary!(">>", INT),
        binary!("&&", BOOL),
        binary!("||", BOOL),
        binary!("^^", BOOL),
        binary!("==", ANY, Terminal::Bool),
        binary!("!=", ANY, Terminal::Bool),
        binary!("<=", NUM, Terminal::Bool),
        binary!(">=", NUM, Terminal::Bool),
        binary!("<", NUM, Terminal::Bool),
        binary!(">", NUM, Terminal::Bool),
    ]
    .into_iter()
    .collect()
}
