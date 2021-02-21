use super::common::Id;
use super::generate::Arg;
use super::generate::BinaryOp;
use super::generate::Data;
use super::generate::Def;
use super::generate::Instruction;
use super::generate::Param;
use super::generate::Target;
use super::generate::UnaryOp;
use super::symbol::Terminal;
use std::fmt;

pub fn emit(target: Target) -> String {
    let mut code = String::new();
    for def in &target.defs {
        code.push_str(&emit_def(&def));
    }
    code
}

fn emit_def(def: &Def) -> String {
    format!(
        "define {} @{}{}({}) {{\n{}}}\n",
        def.ret,
        def.name.name,
        emit_id(def.name.id),
        emit_params(&def.params),
        emit_instructions(&def.instructions)
    )
}

fn emit_params(params: &[Param]) -> String {
    let mut code = String::new();
    if let Some((first, rest)) = params.split_first() {
        code.push_str(&format!("{} %{}", first.typ, first.id));
        for param in rest {
            code.push_str(&format!(", {} %{}", param.typ, param.id));
        }
    }
    code
}

fn emit_args(args: &[Arg]) -> String {
    let mut code = String::new();
    if let Some((first, rest)) = args.split_first() {
        code.push_str(&format!("{} {}", first.typ, first.data));
        for arg in rest {
            code.push_str(&format!(", {} {}", arg.typ, arg.data));
        }
    }
    code
}

fn emit_instructions(instructions: &[Instruction]) -> String {
    let mut code = String::new();
    for instruction in instructions {
        code.push_str(&emit_instruction(instruction));
    }
    code
}

fn emit_instruction(instruction: &Instruction) -> String {
    match instruction {
        Instruction::Ret(ret) => {
            if let Some(data) = &ret.data {
                format!("ret {} {}\n", ret.typ, data)
            } else {
                format!("ret {}\n", ret.typ)
            }
        }
        Instruction::Call(call) => {
            if let Some(out) = &call.out {
                format!(
                    "%{} = call {} @{}{}({})\n",
                    out,
                    call.typ,
                    call.call_name.name,
                    emit_id(call.call_name.id),
                    emit_args(&call.args)
                )
            } else {
                format!(
                    "call {} @{}{}({})\n",
                    call.typ,
                    call.call_name.name,
                    emit_id(call.call_name.id),
                    emit_args(&call.args)
                )
            }
        }
        Instruction::Unary(unary) => match unary.op {
            UnaryOp::Not => {
                format!("%{} = xor {} {}, true\n", unary.out, unary.typ, unary.arg,)
            }
            UnaryOp::BNot => {
                format!("%{} = xor {} {}, -1\n", unary.out, unary.typ, unary.arg,)
            }
        },
        Instruction::Binary(binary) => {
            format!(
                "%{} = {} {} {}, {}\n",
                binary.out,
                emit_binary_op(binary.op, binary.typ),
                binary.typ,
                binary.arg1,
                binary.arg2
            )
        }
    }
}

fn emit_id(id: Id) -> String {
    if id == 0 {
        String::new()
    } else {
        format!("{}", id)
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&match self {
            Data::Id(id) => format!("%{}", id),
            Data::Literal(literal) => literal.to_string(),
        })
    }
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Terminal::Void => "void",
            Terminal::Bool => "i1",
            Terminal::U8 => "u8",
            Terminal::U16 => "u16",
            Terminal::U32 => "u32",
            Terminal::U64 => "u64",
            Terminal::I8 => "i8",
            Terminal::I16 => "i16",
            Terminal::I32 => "i32",
            Terminal::I64 => "i64",
            Terminal::F16 => "half",
            Terminal::F32 => "float",
            Terminal::F64 => "double",
        })
    }
}

fn emit_binary_op(op: BinaryOp, typ: Terminal) -> String {
    enum IntType {
        Unsigned,
        Signed,
        Float,
    }

    fn int_type(typ: Terminal) -> IntType {
        match typ {
            Terminal::I8 | Terminal::I16 | Terminal::I32 | Terminal::I64 => IntType::Signed,
            Terminal::F16 | Terminal::F32 | Terminal::F64 => IntType::Float,
            _ => IntType::Unsigned,
        }
    }

    let typ = int_type(typ);

    macro_rules! instruction {
        ($int_type:ident, $unsigned:literal, $signed:literal, $float:literal) => {
            match $int_type {
                IntType::Unsigned => $unsigned,
                IntType::Signed => $signed,
                IntType::Float => $float,
            }
        };
    }

    match op {
        BinaryOp::Add => instruction!(typ, "add", "add", "fadd"),
        BinaryOp::Sub => instruction!(typ, "sub", "sub", "fsub"),
        BinaryOp::Mul => instruction!(typ, "mul", "mul", "fmul"),
        BinaryOp::Div => instruction!(typ, "udiv", "sdiv", "fdiv"),
        BinaryOp::Rem => instruction!(typ, "urem", "srem", "frem"),
        BinaryOp::BitAnd => instruction!(typ, "and", "and", "and"),
        BinaryOp::BitOr => instruction!(typ, "or", "or", "or"),
        BinaryOp::BitXor => instruction!(typ, "xor", "xor", "xor"),
        BinaryOp::LShift => instruction!(typ, "shl", "shl", "shl"),
        BinaryOp::RShift => instruction!(typ, "lshr", "lshr", "lshr"),
        BinaryOp::And => instruction!(typ, "and", "and", "and"),
        BinaryOp::Or => instruction!(typ, "or", "or", "or"),
        BinaryOp::Xor => instruction!(typ, "xor", "xor", "xor"),
        BinaryOp::Equal => instruction!(typ, "icmp eq", "icmp eq", "fcmp oeq"),
        BinaryOp::NEqual => instruction!(typ, "icmp ne", "icmp ne", "fcmp une"),
        BinaryOp::LEqual => instruction!(typ, "icmp ule", "icmp sle", "fcmp ole"),
        BinaryOp::GEqual => instruction!(typ, "icmp uge", "icmp sge", "fcmp oge"),
        BinaryOp::Less => instruction!(typ, "icmp ult", "icmp slt", "icmp olt"),
        BinaryOp::Greater => instruction!(typ, "icmp ugt", "icmp sgt", "icmp ogt"),
    }
    .to_string()
}
