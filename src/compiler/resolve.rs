use super::common::Location;
use super::error::Error;
use super::namespace::Namespace;
use super::parse::Program;
use super::symbol::Func;
use super::symbol::Symbol;
use super::symbol::Terminal;
use super::symbol::Type;
use std::collections::HashMap;

pub fn resolve(
    mut program: Program,
    mut namespace: Namespace,
) -> Result<(Program, Namespace), Error> {
    for def in &mut program.defs {
        let mut def_namespace = HashMap::new();

        let mut params = Vec::new();
        for param in &def.func.params {
            let param_name = param.name.name.clone();

            let param_type = get_terminal(&param.typ.name, param.typ.location, &namespace)?;
            params.push(param_type);

            let symbol = Symbol::Var(Type::Terminal(param_type));
            def_namespace.insert(param_name, vec![Namespace::from(symbol)]);
        }

        let ret = get_terminal(&def.func.ret.name, def.func.ret.location, &namespace)?;

        let symbol = Symbol::Var(Type::Func(Func { params, ret }));
        let def_namespace = Namespace::from((symbol, def_namespace));
        let id = namespace.append_namespace(&def.name.name, def_namespace);
        def.name.id = id;
    }

    Ok((program, namespace))
}

fn get_terminal(typ: &str, location: Location, namespace: &Namespace) -> Result<Terminal, Error> {
    let namespaces = if let Some(namespaces) = namespace.get(typ) {
        namespaces
    } else {
        return err!(expected_defined_type, location, typ);
    };

    if let Some(Symbol::Type(Type::Terminal(terminal))) =
        namespaces.get(0).map(|namespace| namespace.symbol())
    {
        Ok(*terminal)
    } else {
        err!(expected_terminal_type, location)
    }
}
