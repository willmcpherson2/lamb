use super::common::Location;

#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum Token {
    Open(Location),
    Close(Location),
    Other((String, Location)),
}

pub fn lex(text: &str) -> TokenStream {
    let mut tokens = Vec::new();

    enum State {
        Ready,
        Other,
    }
    let mut state = State::Ready;

    for (location, ch) in text.char_indices() {
        match ch {
            '(' => {
                state = State::Ready;
                tokens.push(Token::Open(location));
            }
            ')' => {
                state = State::Ready;
                tokens.push(Token::Close(location));
            }
            _ if ch.is_whitespace() => {
                state = State::Ready;
            }
            _ => {
                if let State::Ready = state {
                    tokens.push(Token::Other((String::from(ch), location)));
                } else {
                    if let Some(Token::Other((token, _))) = tokens.last_mut() {
                        token.push(ch);
                    } else {
                        panic!();
                    }
                }
                state = State::Other;
            }
        }
    }

    TokenStream { tokens }
}
