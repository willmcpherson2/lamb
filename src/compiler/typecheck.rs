use super::common::Location;
use super::error::Error;
use super::namespace::Namespace;
use super::parse::Call;
use super::parse::Expr;
use super::parse::NameId;
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
        let def_namespace = namespace
            .get_then(&def.name_id.token, def.name_id.id)
            .unwrap();

        if let Symbol::Type(Type::Terminal(ret)) =
            namespace.get_then(&def.func.ret.token, 0).unwrap().symbol()
        {
            typecheck_expr(&mut def.expr, *ret, &namespace, def_namespace)?;
        } else {
            panic!()
        }
    }
    Ok((program, namespace))
}

fn typecheck_main(namespace: &Namespace) -> Result<(), Error> {
    let namespaces = namespace.get("main").ok_or_else(|| error!(expected_main))?;

    let symbol = if let [namespace] = &namespaces[..] {
        namespace.symbol()
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
    outer_ret: Terminal,
    namespace: &Namespace,
    def_namespace: &Namespace,
) -> Result<(), Error> {
    match expr {
        Expr::Val(NameId {
            token, location, ..
        }) => typecheck_val(token, *location, outer_ret, namespace, def_namespace),
        Expr::Call(Call { exprs, location }) => {
            typecheck_exprs(exprs, *location, outer_ret, namespace, def_namespace)
        }
    }
}

fn typecheck_val(
    token: &str,
    token_location: Location,
    outer_ret: Terminal,
    namespace: &Namespace,
    def_namespace: &Namespace,
) -> Result<(), Error> {
    let namespaces = def_namespace
        .get_or(namespace, token)
        .ok_or_else(|| error!(expected_defined_symbol, token_location, &token))?;

    for namespace in namespaces {
        let terminal = match namespace.symbol() {
            Symbol::Literal(terminal) | Symbol::Var(Type::Terminal(terminal)) => terminal,
            _ => return err!(expected_literal_or_var, token_location),
        };

        if *terminal == outer_ret {
            return Ok(());
        }
    }
    err!(type_mismatch, token_location, outer_ret)
}

fn typecheck_exprs(
    exprs: &mut [Expr],
    exprs_location: Location,
    outer_ret: Terminal,
    namespace: &Namespace,
    def_namespace: &Namespace,
) -> Result<(), Error> {
    let parent_expr = if let Some(expr) = exprs.first() {
        expr
    } else {
        if outer_ret != Terminal::Void {
            return err!(type_mismatch, exprs_location, outer_ret, Terminal::Void);
        }
        return Ok(());
    };

    let (parent_token, parent_location) = if let Expr::Val(NameId {
        token, location, ..
    }) = parent_expr
    {
        (token, *location)
    } else {
        return err!(expected_func, exprs_location);
    };

    let namespaces = def_namespace
        .get_or(namespace, parent_token)
        .ok_or_else(|| error!(expected_defined_symbol, parent_location, parent_token))?;

    if let [n] = &namespaces[..] {
        return typecheck_call(
            n.symbol(),
            outer_ret,
            exprs,
            namespace,
            def_namespace,
            parent_location,
        );
    }

    let mut called_id = None;
    for (symbol_id, n) in namespaces.iter().enumerate().rev() {
        if typecheck_call(
            n.symbol(),
            outer_ret,
            exprs,
            namespace,
            def_namespace,
            parent_location,
        )
        .is_ok()
        {
            called_id = Some(symbol_id);
            break;
        }
    }

    let called_id = called_id.ok_or_else(|| error!(no_type_match, parent_location))?;

    if let Expr::Val(NameId { id, .. }) = exprs.first_mut().unwrap() {
        *id = called_id;
        Ok(())
    } else {
        panic!()
    }
}

fn typecheck_call(
    func_symbol: &Symbol,
    outer_ret: Terminal,
    arg_exprs: &mut [Expr],
    namespace: &Namespace,
    def_namespace: &Namespace,
    location: Location,
) -> Result<(), Error> {
    let func = if let Symbol::Var(Type::Func(func)) = func_symbol {
        func
    } else {
        return err!(expected_func, location);
    };

    if func.ret != outer_ret {
        return err!(func_type_mismatch, location, outer_ret, func.ret);
    }

    let mut params = func.params.iter();
    let mut args = arg_exprs.iter_mut().skip(1);
    loop {
        match (params.next(), args.next()) {
            (None, None) => return Ok(()),
            (None, Some(arg)) => match arg {
                Expr::Val(NameId { location, .. }) | Expr::Call(Call { location, .. }) => {
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
