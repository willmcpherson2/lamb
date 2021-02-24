use super::common::Id;
use super::generate::Arg;
use super::generate::BinaryOp;
use super::generate::Def;
use super::generate::Instruction;
use super::generate::Param;
use super::generate::Target;
use super::generate::UnaryOp;
use super::generate::Val;
use super::symbol::Terminal;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub fn emit(target: Target) -> String {
    format!("{}", target)
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for def in &self.defs {
            write!(f, "{}", def)?;
        }
        Ok(())
    }
}

impl Display for Def {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            "define {} @{}({}) {{\n{}}}",
            self.ret,
            IdName(&self.name.token, self.name.id),
            Params(&self.params),
            Instructions(&self.instructions),
        )
    }
}

struct Params<'a>(&'a Vec<Param>);

impl Display for Params<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some((first, rest)) = self.0.split_first() {
            write!(f, "{} %{}", first.typ, first.id)?;
            for param in rest {
                write!(f, ", {} %{}", param.typ, param.id)?;
            }
        }
        Ok(())
    }
}

struct Args<'a>(&'a Vec<Arg>);

impl Display for Args<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some((first, rest)) = self.0.split_first() {
            write!(f, "{} {}", first.typ, first.val)?;
            for arg in rest {
                write!(f, ", {} {}", arg.typ, arg.val)?;
            }
        }
        Ok(())
    }
}

struct Instructions<'a>(&'a Vec<Instruction>);

impl Display for Instructions<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for instruction in self.0 {
            write!(f, "{}", instruction)?;
        }
        Ok(())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Instruction::Ret(ret) => {
                if let Some(val) = &ret.val {
                    writeln!(f, "ret {} {}", ret.typ, val)
                } else {
                    writeln!(f, "ret {}", ret.typ)
                }
            }
            Instruction::Call(call) => {
                if let Some(id) = &call.id {
                    writeln!(
                        f,
                        "%{} = call {} @{}({})",
                        id,
                        call.typ,
                        IdName(&call.called_name.token, call.called_name.id),
                        Args(&call.args)
                    )
                } else {
                    writeln!(
                        f,
                        "call {} @{}({})",
                        call.typ,
                        IdName(&call.called_name.token, call.called_name.id),
                        Args(&call.args)
                    )
                }
            }
            Instruction::Unary(unary) => match unary.op {
                UnaryOp::Not => {
                    writeln!(f, "%{} = xor {} {}, true", unary.id, unary.typ, unary.arg)
                }
                UnaryOp::BitNot => {
                    writeln!(f, "%{} = xor {} {}, -1", unary.id, unary.typ, unary.arg)
                }
            },
            Instruction::Binary(binary) => {
                writeln!(
                    f,
                    "%{} = {} {} {}, {}",
                    binary.id,
                    Op(binary.op, binary.typ),
                    binary.typ,
                    binary.arg1,
                    binary.arg2
                )
            }
        }
    }
}

struct IdName<'a>(&'a str, Id);

impl Display for IdName<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.0)?;
        if self.1 != 0 {
            write!(f, "{}", self.1)?;
        }
        Ok(())
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Val::Id(id) => write!(f, "%{}", *id),
            Val::Literal(literal) => f.write_str(literal),
        }
    }
}

impl Display for Terminal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

struct Op(BinaryOp, Terminal);

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

        let typ = int_type(self.1);

        macro_rules! instruction {
            ($int_type:ident, $unsigned:literal, $signed:literal, $float:literal) => {
                match $int_type {
                    IntType::Unsigned => $unsigned,
                    IntType::Signed => $signed,
                    IntType::Float => $float,
                }
            };
        }

        f.write_str(match self.0 {
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
        })
    }
}
