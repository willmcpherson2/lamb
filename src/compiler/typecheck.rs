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
            if let Some(symbol) = def_namespace.get(&token).or_else(|| namespace.get(&token)) {
                let terminal = if let Symbol::Literal(terminal) = symbol {
                    terminal
                } else if let Symbol::Var(Type::Terminal(terminal)) = symbol {
                    terminal
                } else {
                    return err!("expected literal or var", *location);
                };

                if *terminal != ret {
                    return err!("type mismatch", *location, ret, *terminal);
                }
                Ok(())
            } else {
                err!("expected defined symbol", *location, &token)
            }
        }
        Expr::Expr((exprs, location)) => {
            if let Some(expr) = exprs.first() {
                if let Expr::Val((token, location)) = expr {
                    if let Some(symbol) =
                        def_namespace.get(&token).or_else(|| namespace.get(&token))
                    {
                        if let Symbol::Var(Type::Func(func)) = symbol {
                            if func.ret != ret {
                                return err!("func type mismatch", *location, ret, func.ret);
                            }

                            let mut exprs_iter = exprs.iter().skip(1);
                            for terminal in &func.params {
                                if let Some(expr) = exprs_iter.next() {
                                    typecheck_expr(&expr, *terminal, namespace, def_namespace)?;
                                } else {
                                    return err!("expected argument", *location);
                                }
                            }
                            if let Some(expr) = exprs_iter.next() {
                                let location = match expr {
                                    Expr::Val((_, location)) | Expr::Expr((_, location)) => {
                                        location
                                    }
                                };
                                return err!("unexpected argument", *location);
                            }

                            Ok(())
                        } else {
                            err!("expected func", *location)
                        }
                    } else {
                        err!("expected defined symbol", *location, token)
                    }
                } else {
                    err!("expected func", *location)
                }
            } else {
                if ret == Terminal::Void {
                    Ok(())
                } else {
                    err!("type mismatch", *location, ret, Terminal::Void)
                }
            }
        }
    }
}
