use super::common::Id;
use super::namespace::Namespace;
use super::parse;
use super::parse::Expr;
use super::parse::IdName;
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
    pub id: (String, Id),
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
    pub call_id: (String, Id),
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
        let def_id = def.name.name.clone();
        let def_namespace = namespace.get_then(&def.name.name, def.name.id).unwrap();

        let mut params = Vec::new();
        for param in &def.func.params {
            let param_id = id_map.insert(param.name.name.clone());

            let param_type = get_terminal(&param.typ.name, &namespace);

            let param = Param {
                typ: param_type,
                id: param_id,
            };
            params.push(param);
        }

        let ret = get_terminal(&def.func.ret.name, &namespace);

        id_map.add();

        let mut instructions = Vec::new();
        let ret_data = generate_expr(
            &def.expr,
            &mut instructions,
            &mut id_map,
            &namespace,
            def_namespace,
        );
        let ret_instruction = Instruction::Ret(Ret {
            typ: ret,
            data: ret_data,
        });
        instructions.push(ret_instruction);

        let def = Def {
            id: (def_id, def.name.id),
            params,
            instructions,
            ret,
        };
        defs.push(def);

        id_map.reset();
    }

    Target { defs }
}

fn get_terminal(typ: &str, namespace: &Namespace) -> Terminal {
    if let Symbol::Type(Type::Terminal(terminal)) = namespace.get_then(typ, 0).unwrap().symbol() {
        *terminal
    } else {
        panic!()
    }
}

fn generate_expr(
    expr: &Expr,
    instructions: &mut Vec<Instruction>,
    id_map: &mut IdMap,
    namespace: &Namespace,
    def_namespace: &Namespace,
) -> Option<Data> {
    match expr {
        Expr::Val(IdName { name, .. }) => {
            let symbol = def_namespace
                .get_or_then(namespace, &name, 0)
                .unwrap()
                .symbol();

            match symbol {
                Symbol::Var(_) => Some(Data::Id(id_map.get(&name))),
                Symbol::Literal(_) => Some(Data::Literal(name.clone())),
                _ => panic!(),
            }
        }
        Expr::Call(parse::Call { exprs, .. }) => {
            let (parent, parent_id, children) =
                if let Some((parent, children)) = exprs.split_first() {
                    if let Expr::Val(IdName { name, id, .. }) = parent {
                        (name, *id, children)
                    } else {
                        panic!()
                    }
                } else {
                    return None;
                };

            let id = match parent.as_str() {
                "+" => generate_add(instructions, id_map, namespace, def_namespace, children),
                "*" => generate_mul(instructions, id_map, namespace, def_namespace, children),
                "!" => generate_not(instructions, id_map, namespace, def_namespace, children),
                _ => generate_call(
                    instructions,
                    id_map,
                    namespace,
                    def_namespace,
                    children,
                    parent,
                    parent_id,
                ),
            };

            Some(Data::Id(id))
        }
    }
}

fn generate_add(
    instructions: &mut Vec<Instruction>,
    id_map: &mut IdMap,
    namespace: &Namespace,
    def_namespace: &Namespace,
    children: &[Expr],
) -> Id {
    let child1 = children.get(0).unwrap();
    let child2 = children.get(1).unwrap();

    let arg1 = generate_expr(&child1, instructions, id_map, namespace, def_namespace).unwrap();
    let arg2 = generate_expr(&child2, instructions, id_map, namespace, def_namespace).unwrap();

    let out = id_map.add();

    let typ = Terminal::I32;

    let instruction = Instruction::Add(Add {
        out,
        typ,
        arg1,
        arg2,
    });
    instructions.push(instruction);

    out
}

fn generate_mul(
    instructions: &mut Vec<Instruction>,
    id_map: &mut IdMap,
    namespace: &Namespace,
    def_namespace: &Namespace,
    children: &[Expr],
) -> Id {
    let child1 = children.get(0).unwrap();
    let child2 = children.get(1).unwrap();

    let arg1 = generate_expr(&child1, instructions, id_map, namespace, def_namespace).unwrap();
    let arg2 = generate_expr(&child2, instructions, id_map, namespace, def_namespace).unwrap();

    let out = id_map.add();

    let typ = Terminal::F32;

    let instruction = Instruction::Mul(Mul {
        out,
        typ,
        arg1,
        arg2,
    });
    instructions.push(instruction);

    out
}

fn generate_not(
    instructions: &mut Vec<Instruction>,
    id_map: &mut IdMap,
    namespace: &Namespace,
    def_namespace: &Namespace,
    children: &[Expr],
) -> Id {
    let child = children.get(0).unwrap();

    let arg = generate_expr(&child, instructions, id_map, namespace, def_namespace).unwrap();

    let out = id_map.add();

    let instruction = Instruction::Not(Not { out, arg });
    instructions.push(instruction);

    out
}

fn generate_call(
    instructions: &mut Vec<Instruction>,
    id_map: &mut IdMap,
    namespace: &Namespace,
    def_namespace: &Namespace,
    children: &[Expr],
    parent: &str,
    parent_id: Id,
) -> Id {
    let (params, ret) = if let Symbol::Var(Type::Func(Func { params, ret })) =
        namespace.get_then(parent, parent_id).unwrap().symbol()
    {
        (params, ret)
    } else {
        panic!()
    };

    let mut args = Vec::new();
    for (typ, child) in params.iter().zip(children.iter()) {
        let data = generate_expr(&child, instructions, id_map, namespace, def_namespace).unwrap();
        let arg = Arg { typ: *typ, data };
        args.push(arg);
    }

    let call_id = parent.to_string();

    let out = id_map.add();

    let instruction = Instruction::Call(Call {
        out: Some(out),
        typ: *ret,
        call_id: (call_id, parent_id),
        args,
    });
    instructions.push(instruction);

    out
}
