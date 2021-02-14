use super::common::Location;
use super::lex;
use super::lex::TokenStream;
use std::slice;

#[derive(Debug)]
pub enum TokenTree {
    Token((String, Location)),
    Tree((Vec<TokenTree>, Location)),
}

pub fn treeify(token_stream: TokenStream) -> TokenTree {
    let mut token_stream_iter = token_stream.tokens.iter();
    let mut tree = Vec::new();
    while let Some(token_tree) = treeify_impl(&mut token_stream_iter) {
        tree.push(token_tree)
    }
    TokenTree::Tree((tree, 0))
}

fn treeify_impl(token_stream_iter: &mut slice::Iter<'_, lex::Token>) -> Option<TokenTree> {
    if let Some(token) = token_stream_iter.next() {
        match token {
            lex::Token::Open(location) => {
                let mut tree = Vec::new();
                while let Some(token_tree) = treeify_impl(token_stream_iter) {
                    tree.push(token_tree);
                }
                Some(TokenTree::Tree((tree, *location)))
            }
            lex::Token::Close(_) => None,
            lex::Token::Other((ref token, location)) => {
                Some(TokenTree::Token((token.clone(), *location)))
            }
        }
    } else {
        None
    }
}
