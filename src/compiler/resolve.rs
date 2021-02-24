use super::common::Location;
use super::error::Error;
use super::namespace::Namespace;
use super::parse::Decl;
use super::parse::Param;
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
            let (name_token, typ) = match param {
                Param::Decl(Decl { name, typ, .. }) => (Some(name.token.clone()), typ),
                Param::Type(typ) => (None, typ),
            };

            let param_type = get_terminal(&typ.token, typ.location, &namespace)?;
            params.push(param_type);

            if let Some(name) = name_token {
                let symbol = Symbol::Var(Type::Terminal(param_type));
                def_namespace.insert(name, vec![Namespace::from(symbol)]);
            }
        }

        let ret = get_terminal(&def.func.ret.token, def.func.ret.location, &namespace)?;

        let symbol = Symbol::Var(Type::Func(Func { params, ret }));
        let def_namespace = Namespace::from((symbol, def_namespace));
        let id = namespace.append_namespace(&def.name_id.token, def_namespace);
        def.name_id.id = id;
    }

    Ok((program, namespace))
}

fn get_terminal(
    type_token: &str,
    location: Location,
    namespace: &Namespace,
) -> Result<Terminal, Error> {
    let namespaces = namespace
        .get(type_token)
        .ok_or_else(|| error!(expected_defined_type, location, type_token))?;

    if let Some(Symbol::Type(Type::Terminal(terminal))) = namespaces.get(0).map(Namespace::symbol) {
        Ok(*terminal)
    } else {
        err!(expected_terminal_type, location)
    }
}
