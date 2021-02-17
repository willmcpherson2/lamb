use super::lex::Token;
use super::lex::TokenStream;
use super::namespace::Namespace;
use super::symbol::Symbol;
use super::symbol::Terminal;

pub fn literalise(token_stream: TokenStream) -> (TokenStream, Namespace) {
    let mut namespace = Namespace::new_global();

    for token in &token_stream.tokens {
        if let Token::Other((token, _)) = token {
            if let Some(terminal) = literal(&token) {
                namespace.insert(token.clone(), Symbol::Literal(terminal));
            }
        }
    }

    (token_stream, namespace)
}

fn literal(token: &str) -> Option<Terminal> {
    boolean(token)
        .or_else(|| integer(token))
        .or_else(|| float(token))
}

fn boolean(token: &str) -> Option<Terminal> {
    match token {
        "true" | "false" => Some(Terminal::Bool),
        _ => None,
    }
}

fn integer(token: &str) -> Option<Terminal> {
    for ch in token.chars() {
        if !ch.is_digit(10) {
            return None;
        }
    }
    Some(Terminal::I32)
}

fn float(token: &str) -> Option<Terminal> {
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
        Some(Terminal::F32)
    } else {
        None
    }
}
