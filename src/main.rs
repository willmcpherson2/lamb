#![allow(clippy::collapsible_if)]

#[macro_use]
mod error;
mod compiler;

use error::Error;
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let text = match read_text() {
        Ok(text) => text,
        Err(error) => {
            eprint!("{}", error);
            return;
        }
    };

    let code = match compiler::main(&text) {
        Ok(code) => code,
        Err(mut error) => {
            error.add_text(text);
            eprint!("{}", error);
            return;
        }
    };

    if let Err(error) = clang(code) {
        eprint!("{}", error);
    }
}

fn read_text() -> Result<String, Error> {
    let mut args = env::args();
    args.next().ok_or_else(|| error!("expected arg 1"))?;
    let filename = args.next().ok_or_else(|| error!("expected filename"))?;
    fs::read_to_string(&filename).map_err(|_| error!("file error", filename))
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
