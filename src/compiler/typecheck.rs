use super::common::Location;
use super::error::Error;
use super::namespace::Namespace;
use super::parse::Call;
use super::parse::Expr;
use super::parse::IdName;
use super::parse::Program;
use super::symbol::Func;
use super::symbol::Symbol;
use super::symbol::Terminal;
use super::symbol::Type;

pub fn typecheck(
    mut program: Program,
    namespace: Namespace,
) -> Result<(Program, Namespace), Error> {
    typecheck_main(&namespace)?;

    for def in &mut program.defs {
        let def_namespace = namespace.get_then(&def.name.name, def.name.id).unwrap();

        if let Symbol::Type(Type::Terminal(ret)) =
            namespace.get_then(&def.func.ret.name, 0).unwrap().symbol()
        {
            typecheck_expr(&mut def.expr, *ret, &namespace, def_namespace)?;
        } else {
            panic!()
        }
    }
    Ok((program, namespace))
}

fn typecheck_main(namespace: &Namespace) -> Result<(), Error> {
    let namespaces = if let Some(namespaces) = namespace.get("main") {
        namespaces
    } else {
        return err!(expected_main);
    };

    let symbol = if let [symbol] = &namespaces[..] {
        symbol.symbol()
    } else {
        return err!(unexpected_multi_main);
    };

    let main1 = Symbol::Var(Type::Func(Func {
        params: vec![Terminal::I32],
        ret: Terminal::I32,
    }));

    let main2 = Symbol::Var(Type::Func(Func {
        params: vec![],
        ret: Terminal::I32,
    }));

    if *symbol == main1 || *symbol == main2 {
        Ok(())
    } else {
        err!(expected_main_type)
    }
}

fn typecheck_expr(
    expr: &mut Expr,
    ret: Terminal,
    namespace: &Namespace,
    def_namespace: &Namespace,
) -> Result<(), Error> {
    match expr {
        Expr::Val(IdName { name, location, .. }) => {
            typecheck_val(name, *location, ret, namespace, def_namespace)
        }
        Expr::Call(Call { exprs, location }) => {
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
    if let Some(namespaces) = def_namespace.get_or(&namespace, token) {
        let symbol = namespaces.get(0).unwrap().symbol();

        let terminal = match symbol {
            Symbol::Literal(terminal) | Symbol::Var(Type::Terminal(terminal)) => terminal,
            _ => return err!(expected_literal_or_var, location),
        };

        if *terminal != ret {
            return err!(type_mismatch, location, ret, terminal);
        }
        Ok(())
    } else {
        err!(expected_defined_symbol, location, &token)
    }
}

fn typecheck_exprs(
    exprs: &mut [Expr],
    location: Location,
    ret: Terminal,
    namespace: &Namespace,
    def_namespace: &Namespace,
) -> Result<(), Error> {
    let expr = if let Some(expr) = exprs.first() {
        expr
    } else {
        if ret != Terminal::Void {
            return err!(type_mismatch, location, ret, Terminal::Void);
        }
        return Ok(());
    };

    let (name, location) = if let Expr::Val(IdName { name, location, .. }) = expr {
        (name, *location)
    } else {
        return err!(expected_func, location);
    };

    let namespaces = if let Some(namespaces) = def_namespace.get_or(&namespace, &name) {
        namespaces
    } else {
        return err!(expected_defined_symbol, location, name);
    };

    if namespaces.len() == 1 {
        return typecheck_call(
            namespaces.get(0).unwrap().symbol(),
            ret,
            exprs,
            namespace,
            def_namespace,
            location,
        );
    }

    let mut new_id = None;
    for (symbol_id, namespace) in namespaces.iter().enumerate().rev() {
        if typecheck_call(
            namespace.symbol(),
            ret,
            exprs,
            namespace,
            def_namespace,
            location,
        )
        .is_ok()
        {
            new_id = Some(symbol_id);
            break;
        }
    }

    let new_id = if let Some(new_id) = new_id {
        new_id
    } else {
        return err!(no_type_match, location);
    };

    if let Expr::Val(IdName { id, .. }) = exprs.first_mut().unwrap() {
        *id = new_id;
        Ok(())
    } else {
        panic!()
    }
}

fn typecheck_call(
    symbol: &Symbol,
    ret: Terminal,
    exprs: &mut [Expr],
    namespace: &Namespace,
    def_namespace: &Namespace,
    location: Location,
) -> Result<(), Error> {
    let func = if let Symbol::Var(Type::Func(func)) = symbol {
        func
    } else {
        return err!(expected_func, location);
    };

    if func.ret != ret {
        return err!(func_type_mismatch, location, ret, func.ret);
    }

    let mut params = func.params.iter();
    let mut args = exprs.iter_mut().skip(1);
    loop {
        match (params.next(), args.next()) {
            (None, None) => return Ok(()),
            (None, Some(arg)) => match arg {
                Expr::Val(IdName { location, .. }) | Expr::Call(Call { location, .. }) => {
                    return err!(unexpected_argument, *location);
                }
            },
            (Some(_), None) => return err!(expected_argument, location),
            (Some(param), Some(arg)) => {
                typecheck_expr(arg, *param, namespace, def_namespace)?;
            }
        }
    }
}
