use super::common::Id;
use super::namespace::Namespace;
use super::parse;
use super::parse::Expr;
use super::parse::NameId;
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
    pub val: Val,
}

#[derive(Debug)]
pub enum Val {
    Id(Id),
    Literal(String),
}

#[derive(Debug)]
pub struct Name {
    pub token: String,
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
    pub val: Option<Val>,
}

#[derive(Debug)]
pub struct Call {
    pub id: Option<Id>,
    pub typ: Terminal,
    pub called_name: Name,
    pub args: Vec<Arg>,
}

#[derive(Debug)]
pub struct Unary {
    pub id: Id,
    pub op: UnaryOp,
    pub typ: Terminal,
    pub arg: Val,
}

#[derive(Debug)]
pub struct Binary {
    pub id: Id,
    pub op: BinaryOp,
    pub typ: Terminal,
    pub arg1: Val,
    pub arg2: Val,
}

#[derive(Debug, Copy, Clone)]
pub enum Op {
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    Not,
    BitNot,
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
        Self {
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

fn ops() -> HashMap<String, Op> {
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
        unary!("~", BitNot),
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
    ops: &'a HashMap<String, Op>,
}

pub fn generate(program: Program, namespace: Namespace) -> Target {
    let ops = ops();

    let mut defs = Vec::new();
    let mut id_map = IdMap::new();

    for def in &program.defs {
        let def_name_token = def.name_id.token.clone();
        let def_namespace = namespace
            .get_then(&def.name_id.token, def.name_id.id)
            .unwrap();

        let mut params = Vec::new();
        for param in &def.func.params {
            let (param_type_token, param_id) = match param {
                parse::Param::Decl(parse::Decl { name, typ, .. }) => {
                    (&typ.token, id_map.insert(name.token.clone()))
                }
                parse::Param::Type(parse::Type { token: typ, .. }) => (typ, id_map.add()),
            };

            let param_type = get_terminal(param_type_token, &namespace);

            let param = Param {
                typ: param_type,
                id: param_id,
            };
            params.push(param);
        }

        let ret = get_terminal(&def.func.ret.token, &namespace);

        id_map.add();

        let mut instructions = Vec::new();
        let ret_val = generate_expr(
            &def.expr,
            &mut Info {
                instructions: &mut instructions,
                id_map: &mut id_map,
                namespace: &namespace,
                def_namespace,
                ops: &ops,
            },
        );
        let ret_instruction = Instruction::Ret(Ret {
            typ: ret,
            val: ret_val,
        });
        instructions.push(ret_instruction);

        let def = Def {
            name: Name {
                token: def_name_token,
                id: def.name_id.id,
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

fn get_terminal(type_token: &str, namespace: &Namespace) -> Terminal {
    if let Symbol::Type(Type::Terminal(terminal)) =
        namespace.get_then(type_token, 0).unwrap().symbol()
    {
        *terminal
    } else {
        panic!()
    }
}

fn generate_expr(expr: &Expr, info: &mut Info) -> Option<Val> {
    match expr {
        Expr::Val(NameId { token, .. }) => Some(generate_val(token, info)),
        Expr::Call(parse::Call { exprs, .. }) => Some(Val::Id(generate_call(exprs, info)?)),
    }
}

fn generate_val(token: &str, info: &mut Info) -> Val {
    let symbol = info
        .def_namespace
        .get_or_then(info.namespace, token, 0)
        .unwrap()
        .symbol();

    match symbol {
        Symbol::Var(_) => Val::Id(info.id_map.get(token)),
        Symbol::Literal(_) => Val::Literal(token.to_string()),
        _ => panic!(),
    }
}

fn generate_call(exprs: &[Expr], info: &mut Info) -> Option<Id> {
    let (parent, children) = exprs.split_first()?;

    let (parent_token, parent_id) = if let Expr::Val(NameId { token, id, .. }) = parent {
        (token, *id)
    } else {
        panic!()
    };

    let typ = if let Symbol::Var(Type::Func(Func { ret, .. })) = info
        .namespace
        .get_then(parent_token, parent_id)
        .unwrap()
        .symbol()
    {
        *ret
    } else {
        panic!()
    };

    match info.ops.get(parent_token.as_str()) {
        Some(op) => match op {
            Op::UnaryOp(op) => Some(generate_unary(*op, typ, children, info)),
            Op::BinaryOp(op) => Some(generate_binary(*op, typ, children, info)),
        },
        None => generate_func_call(parent_token, parent_id, children, info),
    }
}

fn generate_unary(op: UnaryOp, typ: Terminal, children: &[Expr], info: &mut Info) -> Id {
    let child = children.get(0).unwrap();

    let arg = generate_expr(child, info).unwrap();

    let id = info.id_map.add();

    let instruction = Instruction::Unary(Unary { op, id, typ, arg });
    info.instructions.push(instruction);

    id
}

fn generate_binary(op: BinaryOp, typ: Terminal, children: &[Expr], info: &mut Info) -> Id {
    let child1 = children.get(0).unwrap();
    let child2 = children.get(1).unwrap();

    let arg1 = generate_expr(child1, info).unwrap();
    let arg2 = generate_expr(child2, info).unwrap();

    let id = info.id_map.add();

    let instruction = Instruction::Binary(Binary {
        op,
        id,
        typ,
        arg1,
        arg2,
    });
    info.instructions.push(instruction);

    id
}

fn generate_func_call(
    parent: &str,
    parent_id: Id,
    children: &[Expr],
    info: &mut Info,
) -> Option<Id> {
    let (params, ret) = if let Symbol::Var(Type::Func(Func { params, ret })) =
        info.namespace.get_then(parent, parent_id).unwrap().symbol()
    {
        (params, ret)
    } else {
        panic!()
    };

    let mut args = Vec::new();
    for (typ, child) in params.iter().zip(children.iter()) {
        let val = generate_expr(child, info).unwrap();
        let arg = Arg { typ: *typ, val };
        args.push(arg);
    }

    let parent_token = parent.to_string();

    let id = if *ret == Terminal::Void {
        None
    } else {
        Some(info.id_map.add())
    };

    let instruction = Instruction::Call(Call {
        id,
        typ: *ret,
        called_name: Name {
            token: parent_token,
            id: parent_id,
        },
        args,
    });
    info.instructions.push(instruction);

    id
}
