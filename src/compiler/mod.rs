#[macro_use]
pub mod error;
mod common;
mod emit;
mod generate;
mod idmap;
mod lex;
mod literalise;
mod namespace;
mod parse;
mod resolve;
mod symbol;
mod treeify;
mod typecheck;

use error::Error;
use generate::Target;
use lex::TokenStream;
use namespace::Namespace;
use parse::Program;
use treeify::TokenTree;

pub fn main(text: &str) -> Result<String, Error> {
    emit(text)
}

pub fn lex(text: &str) -> TokenStream {
    lex::lex(text)
}

pub fn literalise(text: &str) -> (TokenStream, Namespace) {
    let token_stream = lex(text);
    literalise::literalise(token_stream)
}

pub fn treeify(text: &str) -> (TokenTree, Namespace) {
    let (token_stream, namespace) = literalise(text);
    (treeify::treeify(token_stream), namespace)
}

pub fn parse(text: &str) -> Result<(Program, Namespace), Error> {
    let (token_tree, namespace) = treeify(text);
    parse::parse(token_tree, namespace)
}

pub fn resolve(text: &str) -> Result<(Program, Namespace), Error> {
    let (program, namespace) = parse(text)?;
    resolve::resolve(program, namespace)
}

pub fn typecheck(text: &str) -> Result<(Program, Namespace), Error> {
    let (program, namespace) = resolve(text)?;
    typecheck::typecheck(program, namespace)
}

pub fn generate(text: &str) -> Result<Target, Error> {
    let (program, namespace) = typecheck(text)?;
    Ok(generate::generate(program, namespace))
}

pub fn emit(text: &str) -> Result<String, Error> {
    let target = generate(text)?;
    Ok(emit::emit(target))
}
