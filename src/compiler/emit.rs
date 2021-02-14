use super::generate::Arg;
use super::generate::Data;
use super::generate::Def;
use super::generate::Instruction;
use super::generate::Param;
use super::generate::Target;
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
        "define {} @{}({}) {{\n{}}}\n",
        def.ret,
        def.id,
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
                    "%{} = call {} @{}({})\n",
                    out,
                    call.typ,
                    call.call_id,
                    emit_args(&call.args)
                )
            } else {
                format!(
                    "call {} @{}({})\n",
                    call.typ,
                    call.call_id,
                    emit_args(&call.args)
                )
            }
        }
        Instruction::Add(add) => {
            format!(
                "%{} = add {} {}, {}\n",
                add.out, add.typ, add.arg1, add.arg2
            )
        }
        Instruction::Mul(mul) => {
            format!(
                "%{} = fmul {} {}, {}\n",
                mul.out, mul.typ, mul.arg1, mul.arg2
            )
        }
        Instruction::Not(not) => {
            format!("%{} = xor i1 {}, true\n", not.out, not.arg)
        }
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
            Terminal::I32 => "i32",
            Terminal::F32 => "float",
        })
    }
}
