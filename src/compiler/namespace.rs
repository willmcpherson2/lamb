use super::symbol::Func;
use super::symbol::Symbol;
use super::symbol::Terminal;
use super::symbol::Type;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Namespace {
    namespace: HashMap<String, Symbol>,
}

impl Namespace {
    pub fn new() -> Self {
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
            namespace.insert(key.to_string(), val);
        }

        Namespace { namespace }
    }

    pub fn get(&self, key: &str) -> Option<&Symbol> {
        self.namespace.get(key)
    }

    pub fn insert(&mut self, key: String, val: Symbol) {
        self.namespace.insert(key, val);
    }
}
