use super::common::Id;
use super::namespace::Namespace;
use super::parse::Expr;
use super::parse::Program;
use super::symbol::Func;
use super::symbol::Symbol;
use super::symbol::Terminal;
use super::symbol::Type;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Target {
    pub defs: Vec<Def>,
}

#[derive(Debug)]
pub struct Def {
    pub ret: Terminal,
    pub id: String,
    pub params: Vec<Param>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Param {
    pub typ: Terminal,
    pub id: Id,
}

#[derive(Debug)]
pub struct Arg {
    pub typ: Terminal,
    pub data: Data,
}

#[derive(Debug)]
pub enum Data {
    Id(Id),
    Literal(String),
}

#[derive(Debug)]
pub enum Instruction {
    Ret(Ret),
    Call(Call),
    Add(Add),
    Mul(Mul),
    Not(Not),
}

#[derive(Debug)]
pub struct Ret {
    pub typ: Terminal,
    pub data: Option<Data>,
}

#[derive(Debug)]
pub struct Call {
    pub out: Option<Id>,
    pub typ: Terminal,
    pub call_id: String,
    pub args: Vec<Arg>,
}

#[derive(Debug)]
pub struct Add {
    pub out: Id,
    pub typ: Terminal,
    pub arg1: Data,
    pub arg2: Data,
}

#[derive(Debug)]
pub struct Mul {
    pub out: Id,
    pub typ: Terminal,
    pub arg1: Data,
    pub arg2: Data,
}

#[derive(Debug)]
pub struct Not {
    pub out: Id,
    pub arg: Data,
}

struct IdMap {
    ids: HashMap<String, Id>,
    id_count: Id,
}

impl IdMap {
    fn new() -> Self {
        IdMap {
            ids: HashMap::new(),
            id_count: 0,
        }
    }

    fn get(&self, key: &str) -> Id {
        *self.ids.get(key).unwrap()
    }

    fn insert(&mut self, key: String) -> Id {
        let id = self.id_count;
        self.ids.insert(key, id);
        self.id_count += 1;
        id
    }

    fn add(&mut self) -> Id {
        let id = self.id_count;
        self.id_count += 1;
        id
    }

    fn reset(&mut self) {
        self.id_count = 0;
    }
}

pub fn generate(program: Program, namespace: Namespace) -> Target {
    let mut defs = Vec::new();
    let mut id_map = IdMap::new();

    for def in &program.defs {
        let def_id = def.name.0.clone();

        let mut params = Vec::new();
        for param in &def.func.params {
            let param_id = id_map.insert(param.name.0.clone());

            let param_type =
                if let Some(Symbol::Type(Type::Terminal(terminal))) = namespace.get(&param.typ.0) {
                    terminal
                } else {
                    panic!()
                };

            let param = Param {
                typ: *param_type,
                id: param_id,
            };
            params.push(param);
        }

        let ret =
            if let Some(Symbol::Type(Type::Terminal(terminal))) = namespace.get(&def.func.ret.0) {
                terminal
            } else {
                panic!()
            };

        id_map.add();

        let mut instructions = Vec::new();
        let ret_data = generate_expr(&def.expr, &mut instructions, &mut id_map, &namespace);
        let ret_instruction = Instruction::Ret(Ret {
            typ: *ret,
            data: ret_data,
        });
        instructions.push(ret_instruction);

        let def = Def {
            id: def_id,
            params,
            instructions,
            ret: *ret,
        };
        defs.push(def);

        id_map.reset();
    }

    Target { defs }
}

fn generate_expr(
    expr: &Expr,
    instructions: &mut Vec<Instruction>,
    id_map: &mut IdMap,
    namespace: &Namespace,
) -> Option<Data> {
    match expr {
        Expr::Val((token, _)) => match namespace.get(&token).unwrap() {
            Symbol::Var(_) => Some(Data::Id(id_map.get(&token))),
            Symbol::Literal(_) => Some(Data::Literal(token.clone())),
            _ => panic!(),
        },
        Expr::Expr((exprs, _)) => {
            let (parent, children) = if let Some((parent, children)) = exprs.split_first() {
                if let Expr::Val((parent, _)) = parent {
                    (parent, children)
                } else {
                    panic!()
                }
            } else {
                return None;
            };

            match parent.as_str() {
                "+" => {
                    let child1 = children.get(0).unwrap();
                    let child2 = children.get(1).unwrap();

                    let arg1 = generate_expr(&child1, instructions, id_map, namespace).unwrap();
                    let arg2 = generate_expr(&child2, instructions, id_map, namespace).unwrap();

                    let out = id_map.add();

                    let typ = Terminal::I32;

                    let instruction = Instruction::Add(Add {
                        out,
                        typ,
                        arg1,
                        arg2,
                    });
                    instructions.push(instruction);

                    Some(Data::Id(out))
                }
                "*" => {
                    let child1 = children.get(0).unwrap();
                    let child2 = children.get(1).unwrap();

                    let arg1 = generate_expr(&child1, instructions, id_map, namespace).unwrap();
                    let arg2 = generate_expr(&child2, instructions, id_map, namespace).unwrap();

                    let out = id_map.add();

                    let typ = Terminal::F32;

                    let instruction = Instruction::Mul(Mul {
                        out,
                        typ,
                        arg1,
                        arg2,
                    });
                    instructions.push(instruction);

                    Some(Data::Id(out))
                }
                "!" => {
                    let child = children.get(0).unwrap();

                    let arg = generate_expr(&child, instructions, id_map, namespace).unwrap();

                    let out = id_map.add();

                    let instruction = Instruction::Not(Not { out, arg });
                    instructions.push(instruction);

                    Some(Data::Id(out))
                }
                _ => {
                    let (params, ret) = if let Some(Symbol::Var(Type::Func(Func { params, ret }))) =
                        namespace.get(&parent)
                    {
                        (params, ret)
                    } else {
                        panic!()
                    };

                    let mut args = Vec::new();
                    for (typ, child) in params.iter().zip(children.iter()) {
                        let data = generate_expr(&child, instructions, id_map, namespace).unwrap();
                        let arg = Arg { typ: *typ, data };
                        args.push(arg);
                    }

                    let call_id = parent.clone();

                    let out = id_map.add();

                    let instruction = Instruction::Call(Call {
                        out: Some(out),
                        typ: *ret,
                        call_id,
                        args,
                    });
                    instructions.push(instruction);

                    Some(Data::Id(out))
                }
            }
        }
    }
}
