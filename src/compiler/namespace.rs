use super::common::Id;
use super::symbol::Func;
use super::symbol::Symbol;
use super::symbol::Terminal;
use super::symbol::Type;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Namespace {
    namespace: HashMap<String, Vec<(Symbol, Namespace)>>,
}

impl Namespace {
    pub fn new() -> Self {
        Namespace {
            namespace: HashMap::new(),
        }
    }

    pub fn new_global() -> Self {
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
            namespace.insert(key.to_string(), vec![(val, Namespace::new())]);
        }

        Namespace { namespace }
    }

    pub fn get(&self, key: &str) -> Option<&Vec<(Symbol, Namespace)>> {
        self.namespace.get(key)
    }

    pub fn get_or<'a>(
        &'a self,
        other: &'a Namespace,
        key: &str,
    ) -> Option<&'a Vec<(Symbol, Namespace)>> {
        self.get(key).or_else(|| other.get(key))
    }

    pub fn get_then<'a>(&'a self, key: &str, id: Id) -> Option<&'a (Symbol, Namespace)> {
        self.get(key).and_then(|symbols| symbols.get(id))
    }

    pub fn get_or_then<'a>(
        &'a self,
        other: &'a Namespace,
        key: &str,
        id: Id,
    ) -> Option<&'a (Symbol, Namespace)> {
        self.get_or(other, key).and_then(|symbols| symbols.get(id))
    }

    pub fn insert(&mut self, key: String, val: Symbol) {
        self.namespace.insert(key, vec![(val, Namespace::new())]);
    }

    pub fn insert_with_namespace(&mut self, key: String, val: Symbol, namespace: Namespace) -> Id {
        if let Some(symbols) = self.namespace.get_mut(&key) {
            let id = symbols.len();
            symbols.push((val, namespace));
            id
        } else {
            self.namespace.insert(key, vec![(val, namespace)]);
            0
        }
    }
}
