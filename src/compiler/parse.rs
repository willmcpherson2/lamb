use super::common::Location;
use super::error::Error;
use super::namespace::Namespace;
use super::treeify::TokenTree;

#[derive(Debug)]
pub struct Program {
    pub defs: Vec<Def>,
}

#[derive(Debug)]
pub struct Def {
    pub name: (String, Location),
    pub func: Func,
    pub expr: Expr,
    pub location: Location,
}

#[derive(Debug)]
pub struct Func {
    pub params: Vec<Param>,
    pub ret: (String, Location),
    pub location: Location,
}

#[derive(Debug)]
pub struct Param {
    pub name: (String, Location),
    pub typ: (String, Location),
    pub location: Location,
}

#[derive(Debug)]
pub enum Expr {
    Val((String, Location)),
    Expr((Vec<Expr>, Location)),
}

pub fn parse(token_tree: TokenTree, namespace: Namespace) -> Result<(Program, Namespace), Error> {
    let mut defs = Vec::new();

    match token_tree {
        TokenTree::Token((token, location)) => {
            return err!("expected paren", location, token);
        }
        TokenTree::Tree((tree, _)) => {
            for token_tree in &tree {
                defs.push(parse_def(token_tree)?);
            }
        }
    }

    Ok((Program { defs }, namespace))
}

fn parse_def(token_tree: &TokenTree) -> Result<Def, Error> {
    match token_tree {
        TokenTree::Token((token, location)) => {
            err!("expected def", *location, token)
        }
        TokenTree::Tree((tree, location)) => {
            let (name, name_location) = parse_def_name(tree, *location)?;
            let (func, func_location) = parse_def_func(tree, name_location)?;
            let expr = parse_def_expr(tree, func_location)?;

            if let Some(token_tree) = tree.get(3) {
                let location = match token_tree {
                    TokenTree::Tree((_, location)) | TokenTree::Token((_, location)) => location,
                };
                return err!("unexpected token", *location);
            }

            Ok(Def {
                name: (name, name_location),
                func,
                expr,
                location: *location,
            })
        }
    }
}

fn parse_def_name(tree: &[TokenTree], location: Location) -> Result<(String, Location), Error> {
    if let Some(token_tree) = tree.get(0) {
        match token_tree {
            TokenTree::Tree((_, location)) => err!("expected name", *location),
            TokenTree::Token((token, location)) => Ok((token.clone(), *location)),
        }
    } else {
        err!("expected name", location)
    }
}

fn parse_def_func(tree: &[TokenTree], name_location: Location) -> Result<(Func, Location), Error> {
    if let Some(token_tree) = tree.get(1) {
        match token_tree {
            TokenTree::Tree((tree, location)) => Ok((parse_func(tree, *location)?, *location)),
            TokenTree::Token((_, location)) => err!("expected func type", *location),
        }
    } else {
        err!("expected func type after name", name_location)
    }
}

fn parse_def_expr(tree: &[TokenTree], func_location: Location) -> Result<Expr, Error> {
    if let Some(token_tree) = tree.get(2) {
        Ok(parse_expr(token_tree))
    } else {
        err!("expected func expr", func_location)
    }
}

fn parse_func(tree: &[TokenTree], location: Location) -> Result<Func, Error> {
    let (ret, params) = if let Some(tree) = tree.split_last() {
        tree
    } else {
        return err!("expected type", location);
    };

    let (token, ret_location) = match ret {
        TokenTree::Token(token) => token,
        TokenTree::Tree((_, location)) => {
            return err!("expected func ret terminal type", *location)
        }
    };

    let mut func_params = Vec::new();
    for param in params {
        func_params.push(parse_param(&param)?);
    }

    Ok(Func {
        params: func_params,
        ret: (token.clone(), *ret_location),
        location,
    })
}

fn parse_param(token_tree: &TokenTree) -> Result<Param, Error> {
    let (tree, location) = match token_tree {
        TokenTree::Tree(tree) => tree,
        TokenTree::Token((_, location)) => return err!("expected param", *location),
    };

    let (name, typ) = if let [name, typ] = &tree[..] {
        (name, typ)
    } else {
        return err!("expected param", *location);
    };

    let name = match name {
        TokenTree::Tree((_, location)) => {
            return err!("expected param name", *location);
        }
        TokenTree::Token((token, location)) => (token.clone(), *location),
    };

    let typ = match typ {
        TokenTree::Tree((_, location)) => {
            return err!("expected param type", *location);
        }
        TokenTree::Token((token, location)) => (token.clone(), *location),
    };

    Ok(Param {
        name,
        typ,
        location: *location,
    })
}

fn parse_expr(token_tree: &TokenTree) -> Expr {
    match token_tree {
        TokenTree::Tree((tree, location)) => {
            let mut exprs = Vec::new();
            for expr in tree {
                exprs.push(parse_expr(&expr));
            }
            Expr::Expr((exprs, *location))
        }
        TokenTree::Token((token, location)) => Expr::Val((token.clone(), *location)),
    }
}
