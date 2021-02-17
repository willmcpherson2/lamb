use super::common::Location;
use super::error::Error;
use super::namespace::Namespace;
use super::parse::Expr;
use super::parse::Program;
use super::symbol::Symbol;
use super::symbol::Terminal;
use super::symbol::Type;

pub fn typecheck(program: Program, namespace: Namespace) -> Result<(Program, Namespace), Error> {
    for def in &program.defs {
        let def_namespace = &namespace.get_namespace(&def.name.0).unwrap();
        if let Some(Symbol::Type(Type::Terminal(ret))) = namespace.get(&def.func.ret.0) {
            typecheck_expr(&def.expr, *ret, &namespace, def_namespace)?;
        } else {
            panic!()
        }
    }
    Ok((program, namespace))
}

fn typecheck_expr(
    expr: &Expr,
    ret: Terminal,
    namespace: &Namespace,
    def_namespace: &Namespace,
) -> Result<(), Error> {
    match expr {
        Expr::Val((token, location)) => {
            typecheck_val(token, *location, ret, namespace, def_namespace)
        }
        Expr::Expr((exprs, location)) => {
            typecheck_exprs(exprs, *location, ret, namespace, def_namespace)
        }
    }
}

fn typecheck_val(
    token: &str,
    location: Location,
    ret: Terminal,
    namespace: &Namespace,
    def_namespace: &Namespace,
) -> Result<(), Error> {
    if let Some(symbol) = def_namespace.get(token).or_else(|| namespace.get(token)) {
        let terminal = match symbol {
            Symbol::Literal(terminal) | Symbol::Var(Type::Terminal(terminal)) => terminal,
            _ => return err!("expected literal or var", location),
        };

        if *terminal != ret {
            return err!("type mismatch", location, ret, *terminal);
        }
        Ok(())
    } else {
        err!("expected defined symbol", location, &token)
    }
}

fn typecheck_exprs(
    exprs: &[Expr],
    location: Location,
    ret: Terminal,
    namespace: &Namespace,
    def_namespace: &Namespace,
) -> Result<(), Error> {
    if let Some(expr) = exprs.first() {
        if let Expr::Val((token, location)) = expr {
            if let Some(symbol) = def_namespace.get(&token).or_else(|| namespace.get(&token)) {
                if let Symbol::Var(Type::Func(func)) = symbol {
                    if func.ret != ret {
                        return err!("func type mismatch", *location, ret, func.ret);
                    }

                    let mut params = func.params.iter();
                    let mut args = exprs.iter().skip(1);
                    loop {
                        match (params.next(), args.next()) {
                            (None, None) => return Ok(()),
                            (None, Some(arg)) => match arg {
                                Expr::Val((_, location)) | Expr::Expr((_, location)) => {
                                    return err!("unexpected argument", *location);
                                }
                            },
                            (Some(_), None) => return err!("expected argument", *location),
                            (Some(param), Some(arg)) => {
                                typecheck_expr(arg, *param, namespace, def_namespace)?;
                            }
                        }
                    }
                } else {
                    err!("expected func", *location)
                }
            } else {
                err!("expected defined symbol", *location, token)
            }
        } else {
            err!("expected func", location)
        }
    } else {
        if ret != Terminal::Void {
            return err!("type mismatch", location, ret, Terminal::Void);
        }
        Ok(())
    }
}
