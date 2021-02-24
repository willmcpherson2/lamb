use super::lex::Token;
use super::lex::TokenStream;
use super::namespace::Namespace;
use super::symbol::Symbol;
use super::symbol::Terminal;
use super::symbol::BOOL;
use super::symbol::FLOAT;
use super::symbol::INT;

pub fn literalise(token_stream: TokenStream) -> (TokenStream, Namespace) {
    let mut namespace = Namespace::new_module();

    for token in &token_stream.tokens {
        if let Token::Other(token, _) = token {
            if let Some(terminals) = literal(token) {
                if namespace.get(token).is_none() {
                    let namespaces = terminals
                        .iter()
                        .map(|terminal| Namespace::from(Symbol::Literal(*terminal)))
                        .collect();
                    namespace.insert_namespaces(token.clone(), namespaces);
                }
            }
        }
    }

    (token_stream, namespace)
}

fn literal(token: &str) -> Option<&[Terminal]> {
    boolean(token)
        .or_else(|| integer(token))
        .or_else(|| float(token))
}

fn boolean(token: &str) -> Option<&[Terminal]> {
    match token {
        "true" | "false" => Some(&BOOL),
        _ => None,
    }
}

fn integer(token: &str) -> Option<&[Terminal]> {
    for ch in token.chars() {
        if !ch.is_digit(10) {
            return None;
        }
    }
    Some(&INT)
}

fn float(token: &str) -> Option<&[Terminal]> {
    enum State {
        FirstInteger,
        Integer,
        FirstFraction,
        Fraction,
    }
    let mut state = State::FirstInteger;

    for ch in token.chars() {
        if ch == '.' {
            if let State::Integer = state {
                state = State::FirstFraction;
            } else {
                return None;
            }
        } else if ch.is_digit(10) {
            match state {
                State::FirstInteger => state = State::Integer,
                State::FirstFraction => state = State::Fraction,
                _ => (),
            }
        } else {
            return None;
        }
    }

    if let State::Fraction = state {
        Some(&FLOAT)
    } else {
        None
    }
}
