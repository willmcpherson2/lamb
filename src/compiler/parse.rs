use super::common::Id;
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
    pub name_id: NameId,
    pub func: Func,
    pub expr: Expr,
    pub location: Location,
}

#[derive(Debug)]
pub struct Func {
    pub params: Vec<Param>,
    pub ret: Type,
    pub location: Location,
}

#[derive(Debug)]
pub enum Param {
    Decl(Decl),
    Type(Type),
}

#[derive(Debug)]
pub enum Expr {
    Val(NameId),
    Call(Call),
}

#[derive(Debug)]
pub struct Call {
    pub exprs: Vec<Expr>,
    pub location: Location,
}

#[derive(Debug)]
pub struct Decl {
    pub name: Name,
    pub typ: Type,
    pub location: Location,
}

#[derive(Debug)]
pub struct Name {
    pub token: String,
    pub location: Location,
}

#[derive(Debug)]
pub struct Type {
    pub token: String,
    pub location: Location,
}

#[derive(Debug)]
pub struct NameId {
    pub token: String,
    pub id: Id,
    pub location: Location,
}

pub fn parse(token_tree: TokenTree, namespace: Namespace) -> Result<(Program, Namespace), Error> {
    let mut defs = Vec::new();

    match token_tree {
        TokenTree::Token(token, location) => {
            return err!(expected_paren, location, token);
        }
        TokenTree::Tree(tree, _) => {
            for token_tree in &tree {
                defs.push(parse_def(token_tree)?);
            }
        }
    }

    Ok((Program { defs }, namespace))
}

fn parse_def(token_tree: &TokenTree) -> Result<Def, Error> {
    let (tree, location) = match token_tree {
        TokenTree::Token(token, location) => {
            return err!(expected_def, *location, token);
        }
        TokenTree::Tree(tree, location) => (tree, *location),
    };

    let name_id = parse_def_name(tree, location)?;
    let func = parse_def_func(tree, name_id.location)?;
    let expr = parse_def_expr(tree, func.location)?;

    if let Some(token_tree) = tree.get(3) {
        let location = match token_tree {
            TokenTree::Tree(_, location) | TokenTree::Token(_, location) => location,
        };
        return err!(unexpected_token, *location);
    }

    Ok(Def {
        name_id,
        func,
        expr,
        location,
    })
}

fn parse_def_name(tree: &[TokenTree], tree_location: Location) -> Result<NameId, Error> {
    let token_tree = tree
        .get(0)
        .ok_or_else(|| error!(expected_name, tree_location))?;

    match token_tree {
        TokenTree::Tree(_, location) => err!(expected_name, *location),
        TokenTree::Token(token, location) => Ok(NameId {
            token: token.clone(),
            id: 0,
            location: *location,
        }),
    }
}

fn parse_def_func(tree: &[TokenTree], name_location: Location) -> Result<Func, Error> {
    let token_tree = tree
        .get(1)
        .ok_or_else(|| error!(expected_func_type_after_name, name_location))?;

    match token_tree {
        TokenTree::Tree(tree, location) => parse_func(tree, *location),
        TokenTree::Token(_, location) => err!(expected_func_type, *location),
    }
}

fn parse_def_expr(tree: &[TokenTree], func_location: Location) -> Result<Expr, Error> {
    let token_tree = tree
        .get(2)
        .ok_or_else(|| error!(expected_func_expr, func_location))?;

    Ok(parse_expr(token_tree))
}

fn parse_func(tree: &[TokenTree], tree_location: Location) -> Result<Func, Error> {
    let (ret, params) = tree
        .split_last()
        .ok_or_else(|| error!(expected_type, tree_location))?;

    let (ret_token, ret_location) = match ret {
        TokenTree::Token(token, location) => (token, location),
        TokenTree::Tree(_, location) => return err!(expected_func_ret_terminal_type, *location),
    };

    let mut func_params = Vec::new();
    for param in params {
        func_params.push(parse_param(param)?);
    }

    Ok(Func {
        params: func_params,
        ret: Type {
            token: ret_token.clone(),
            location: *ret_location,
        },
        location: tree_location,
    })
}

fn parse_param(token_tree: &TokenTree) -> Result<Param, Error> {
    match token_tree {
        TokenTree::Tree(tree, location) => parse_decl(tree, *location),
        TokenTree::Token(token, location) => Ok(Param::Type(Type {
            token: token.clone(),
            location: *location,
        })),
    }
}

fn parse_decl(tree: &[TokenTree], tree_location: usize) -> Result<Param, Error> {
    let (name, typ) = if let [name, typ] = &tree[..] {
        (name, typ)
    } else {
        return err!(expected_param, tree_location);
    };

    let name = match name {
        TokenTree::Tree(_, location) => {
            return err!(expected_param_name, *location);
        }
        TokenTree::Token(token, location) => Name {
            token: token.clone(),
            location: *location,
        },
    };

    let typ = match typ {
        TokenTree::Tree(_, location) => {
            return err!(expected_param_type, *location);
        }
        TokenTree::Token(token, location) => Type {
            token: token.clone(),
            location: *location,
        },
    };

    Ok(Param::Decl(Decl {
        name,
        typ,
        location: tree_location,
    }))
}

fn parse_expr(token_tree: &TokenTree) -> Expr {
    match token_tree {
        TokenTree::Tree(tree, location) => {
            let mut exprs = Vec::new();
            for expr in tree {
                exprs.push(parse_expr(expr));
            }
            Expr::Call(Call {
                exprs,
                location: *location,
            })
        }
        TokenTree::Token(token, location) => Expr::Val(NameId {
            token: token.clone(),
            id: 0,
            location: *location,
        }),
    }
}
