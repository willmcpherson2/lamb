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
        let builtins = vec![
            ("void", Symbol::Type(Type::Terminal(Terminal::Void))),
            ("bool", Symbol::Type(Type::Terminal(Terminal::Bool))),
            ("i32", Symbol::Type(Type::Terminal(Terminal::I32))),
            ("f32", Symbol::Type(Type::Terminal(Terminal::F32))),
            (
                "!",
                Symbol::Var(Type::Func(Func {
                    params: vec![Terminal::Bool],
                    ret: Terminal::Bool,
                })),
            ),
            (
                "+",
                Symbol::Var(Type::Func(Func {
                    params: vec![Terminal::I32, Terminal::I32],
                    ret: Terminal::I32,
                })),
            ),
            (
                "*",
                Symbol::Var(Type::Func(Func {
                    params: vec![Terminal::F32, Terminal::F32],
                    ret: Terminal::F32,
                })),
            ),
        ];

        let mut namespace = HashMap::new();
        for (key, val) in builtins.into_iter() {
            namespace.insert(key.to_string(), vec![Namespace::from(val)]);
        }

        Namespace::from((Symbol::Module, namespace))
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
