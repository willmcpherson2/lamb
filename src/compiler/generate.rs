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
    pub name: Name,
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
pub struct Name {
    pub name: String,
    pub id: Id,
}

#[derive(Debug)]
pub enum Instruction {
    Ret(Ret),
    Call(Call),
    Unary(Unary),
    Binary(Binary),
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
    pub call_name: Name,
    pub args: Vec<Arg>,
}

#[derive(Debug)]
pub struct Unary {
    pub out: Id,
    pub op: UnaryOp,
    pub typ: Terminal,
    pub arg: Data,
}

#[derive(Debug)]
pub struct Binary {
    pub out: Id,
    pub op: BinaryOp,
    pub typ: Terminal,
    pub arg1: Data,
    pub arg2: Data,
}

#[derive(Debug, Copy, Clone)]
pub enum Op {
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    Not,
    BNot,
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
    BitAnd,
    BitOr,
    BitXor,
    LShift,
    RShift,
    Equal,
    NEqual,
    LEqual,
    GEqual,
    Less,
    Greater,
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

fn operators() -> HashMap<String, Op> {
    macro_rules! unary {
        ($s:literal, $op:tt) => {
            ($s.to_string(), Op::UnaryOp(UnaryOp::$op))
        };
    }

    macro_rules! binary {
        ($s:literal, $op:tt) => {
            ($s.to_string(), Op::BinaryOp(BinaryOp::$op))
        };
    }

    vec![
        unary!("!", Not),
        unary!("~", BNot),
        binary!("+", Add),
        binary!("-", Sub),
        binary!("*", Mul),
        binary!("/", Div),
        binary!("%", Rem),
        binary!("&", BitAnd),
        binary!("|", BitOr),
        binary!("^", BitXor),
        binary!("<<", LShift),
        binary!(">>", RShift),
        binary!("&&", And),
        binary!("||", Or),
        binary!("^^", Xor),
        binary!("==", Equal),
        binary!("!=", NEqual),
        binary!("<=", LEqual),
        binary!(">=", GEqual),
        binary!("<", Less),
        binary!(">", Greater),
    ]
    .into_iter()
    .collect()
}

struct Info<'a> {
    instructions: &'a mut Vec<Instruction>,
    id_map: &'a mut IdMap,
    namespace: &'a Namespace,
    def_namespace: &'a Namespace,
    operators: &'a HashMap<String, Op>,
}

pub fn generate(program: Program, namespace: Namespace) -> Target {
    let operators = operators();

    let mut defs = Vec::new();
    let mut id_map = IdMap::new();

    for def in &program.defs {
        let def_name = def.name.name.clone();
        let def_namespace = namespace.get_then(&def.name.name, def.name.id).unwrap();

        let mut params = Vec::new();
        for param in &def.func.params {
            let (param_id, typ) = match param {
                parse::Param::NameType(parse::NameType { name, typ, .. }) => {
                    (id_map.insert(name.name.clone()), &typ.typ)
                }
                parse::Param::Type(parse::Type { typ, .. }) => (id_map.add(), typ),
            };

            let param_type = get_terminal(typ, &namespace);

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
            &mut Info {
                instructions: &mut instructions,
                id_map: &mut id_map,
                namespace: &namespace,
                def_namespace,
                operators: &operators,
            },
        );
        let ret_instruction = Instruction::Ret(Ret {
            typ: ret,
            data: ret_data,
        });
        instructions.push(ret_instruction);

        let def = Def {
            name: Name {
                name: def_name,
                id: def.name.id,
            },
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

fn generate_expr(expr: &Expr, info: &mut Info) -> Option<Data> {
    match expr {
        Expr::Val(IdName { name, .. }) => {
            let symbol = info
                .def_namespace
                .get_or_then(info.namespace, &name, 0)
                .unwrap()
                .symbol();

            match symbol {
                Symbol::Var(_) => Some(Data::Id(info.id_map.get(&name))),
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

            let typ = if let Symbol::Var(Type::Func(Func { ret, .. })) =
                info.namespace.get_then(parent, parent_id).unwrap().symbol()
            {
                *ret
            } else {
                panic!()
            };

            let id = match info.operators.get(parent.as_str()) {
                Some(op) => match op {
                    Op::UnaryOp(op) => generate_unary(children, typ, *op, info),
                    Op::BinaryOp(op) => generate_binary(children, typ, *op, info),
                },
                None => generate_call(children, parent, parent_id, info),
            };

            Some(Data::Id(id))
        }
    }
}

fn generate_unary(children: &[Expr], typ: Terminal, op: UnaryOp, info: &mut Info) -> Id {
    let child = children.get(0).unwrap();

    let arg = generate_expr(&child, info).unwrap();

    let out = info.id_map.add();

    let instruction = Instruction::Unary(Unary { op, out, typ, arg });
    info.instructions.push(instruction);

    out
}

fn generate_binary(children: &[Expr], typ: Terminal, op: BinaryOp, info: &mut Info) -> Id {
    let child1 = children.get(0).unwrap();
    let child2 = children.get(1).unwrap();

    let arg1 = generate_expr(&child1, info).unwrap();
    let arg2 = generate_expr(&child2, info).unwrap();

    let out = info.id_map.add();

    let instruction = Instruction::Binary(Binary {
        op,
        out,
        typ,
        arg1,
        arg2,
    });
    info.instructions.push(instruction);

    out
}

fn generate_call(children: &[Expr], parent: &str, parent_id: Id, info: &mut Info) -> Id {
    let (params, ret) = if let Symbol::Var(Type::Func(Func { params, ret })) =
        info.namespace.get_then(parent, parent_id).unwrap().symbol()
    {
        (params, ret)
    } else {
        panic!()
    };

    let mut args = Vec::new();
    for (typ, child) in params.iter().zip(children.iter()) {
        let data = generate_expr(&child, info).unwrap();
        let arg = Arg { typ: *typ, data };
        args.push(arg);
    }

    let call_id = parent.to_string();

    let out = info.id_map.add();

    let instruction = Instruction::Call(Call {
        out: Some(out),
        typ: *ret,
        call_name: Name {
            name: call_id,
            id: parent_id,
        },
        args,
    });
    info.instructions.push(instruction);

    out
}
