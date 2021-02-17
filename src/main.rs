#![allow(clippy::collapsible_if)]

#[macro_use]
mod error;
mod compiler;

use error::Error;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let (text, args) = match read_text() {
        Ok(text) => text,
        Err(error) => {
            error.print();
            return;
        }
    };

    if dbg_pipeline(&text, args) {
        return;
    }

    let code = match compiler::main(&text) {
        Ok(code) => code,
        Err(error) => {
            error.print(&text);
            return;
        }
    };

    if let Err(error) = clang(code) {
        error.print();
    }
}

fn read_text() -> Result<(String, HashSet<String>), Error> {
    let mut filename = None;
    let mut args = HashSet::new();
    for arg in env::args().skip(1) {
        if arg.starts_with('-') {
            args.insert(arg);
        } else {
            filename = Some(arg);
        }
    }
    let filename = filename.ok_or_else(|| error!("expected filename"))?;
    let text = fs::read_to_string(&filename).map_err(|_| error!("file error", filename))?;
    Ok((text, args))
}

fn clang(code: String) -> Result<(), Error> {
    let mut clang = Command::new("clang")
        .args(&["-x", "ir", "-"])
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|_| error!("clang spawn failed"))?;

    let clang_in = clang
        .stdin
        .as_mut()
        .ok_or_else(|| error!("clang stdin failed"))?;

    clang_in
        .write_all(code.as_bytes())
        .map_err(|_| error!("clang write failed"))?;

    let clang_out = clang
        .wait_with_output()
        .map_err(|_| error!("clang output failed"))?;

    eprint!("{}", String::from_utf8_lossy(&clang_out.stderr));

    let clang_status = clang_out
        .status
        .code()
        .ok_or_else(|| error!("clang status failed"))?;

    if clang_status == 0 {
        Ok(())
    } else {
        err!("clang non-zero", clang_status)
    }
}

fn dbg_pipeline(text: &str, args: HashSet<String>) -> bool {
    macro_rules! dbg_pipeline {
        ($text:ident, $args:ident, $arg:literal, $stage:path) => {
            if $args.contains($arg) {
                dbg!($stage($text));
                return true;
            }
        };
    }

    macro_rules! dbg_pipeline_err {
        ($text:ident, $args:ident, $arg:literal, $stage:path) => {
            if $args.contains($arg) {
                match $stage($text) {
                    Ok(code) => {
                        dbg!(code);
                    }
                    Err(error) => {
                        error.print(text);
                    }
                };
                return true;
            }
        };
    }

    dbg_pipeline!(text, args, "--lex", compiler::lex);
    dbg_pipeline!(text, args, "--literalise", compiler::literalise);
    dbg_pipeline!(text, args, "--treeify", compiler::treeify);
    dbg_pipeline_err!(text, args, "--parse", compiler::parse);
    dbg_pipeline_err!(text, args, "--resolve", compiler::resolve);
    dbg_pipeline_err!(text, args, "--typecheck", compiler::typecheck);
    dbg_pipeline_err!(text, args, "--generate", compiler::generate);
    dbg_pipeline_err!(text, args, "--emit", compiler::emit);

    false
}
