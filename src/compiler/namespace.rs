use super::common::Id;
use super::symbol::Func;
use super::symbol::Symbol;
use super::symbol::Terminal;
use super::symbol::Type;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Namespace {
    symbol: Symbol,
    namespace: HashMap<String, Vec<Namespace>>,
}

impl From<(Symbol, HashMap<String, Vec<Namespace>>)> for Namespace {
    fn from(parts: (Symbol, HashMap<String, Vec<Namespace>>)) -> Self {
        Namespace {
            symbol: parts.0,
            namespace: parts.1,
        }
    }
}

impl From<Symbol> for Namespace {
    fn from(symbol: Symbol) -> Self {
        Namespace {
            symbol,
            namespace: HashMap::new(),
        }
    }
}

impl Namespace {
    pub fn new_module() -> Self {
        Namespace::from((Symbol::Module, builtins()))
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    pub fn get(&self, key: &str) -> Option<&Vec<Namespace>> {
        self.namespace.get(key)
    }

    pub fn get_or<'a>(&'a self, other: &'a Namespace, key: &str) -> Option<&'a Vec<Namespace>> {
        self.get(key).or_else(|| other.get(key))
    }

    pub fn get_then<'a>(&'a self, key: &str, id: Id) -> Option<&'a Namespace> {
        self.get(key).and_then(|symbols| symbols.get(id))
    }

    pub fn get_or_then<'a>(
        &'a self,
        other: &'a Namespace,
        key: &str,
        id: Id,
    ) -> Option<&'a Namespace> {
        self.get_or(other, key).and_then(|symbols| symbols.get(id))
    }

    pub fn insert(&mut self, key: String, val: Symbol) {
        self.namespace.insert(key, vec![Namespace::from(val)]);
    }

    pub fn insert_namespace(&mut self, key: String, namespace: Namespace) -> Id {
        if let Some(namespaces) = self.namespace.get_mut(&key) {
            let id = namespaces.len();
            namespaces.push(namespace);
            id
        } else {
            self.namespace.insert(key, vec![namespace]);
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

    const BOOL: [Terminal; 1] = [Terminal::Bool];

    const INT: [Terminal; 8] = [
        Terminal::U8,
        Terminal::U16,
        Terminal::U32,
        Terminal::U64,
        Terminal::I8,
        Terminal::I16,
        Terminal::I32,
        Terminal::I64,
    ];

    const NUM: [Terminal; 11] = [
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

    const ANY: [Terminal; 12] = [
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
