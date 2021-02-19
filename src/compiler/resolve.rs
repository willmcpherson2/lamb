use super::common::Location;
use super::error::Error;
use super::namespace::Namespace;
use super::parse::Program;
use super::symbol::Func;
use super::symbol::Symbol;
use super::symbol::Terminal;
use super::symbol::Type;

pub fn resolve(
    mut program: Program,
    mut namespace: Namespace,
) -> Result<(Program, Namespace), Error> {
    for def in &mut program.defs {
        let name = def.name.0.clone();
        let mut def_namespace = Namespace::new();

        let mut params = Vec::new();
        for param in &def.func.params {
            let param_name = param.name.0.clone();

            let param_type = get_terminal(&param.typ.0, param.typ.1, &namespace)?;
            params.push(param_type);

            let symbol = Symbol::Var(Type::Terminal(param_type));
            def_namespace.insert(param_name, symbol);
        }

        let ret = get_terminal(&def.func.ret.0, def.func.ret.1, &namespace)?;

        let symbol = Symbol::Var(Type::Func(Func { params, ret }));
        let id = namespace.insert_with_namespace(name, symbol, def_namespace);
        def.name.2 = id;
    }

    Ok((program, namespace))
}

fn get_terminal(typ: &str, location: Location, namespace: &Namespace) -> Result<Terminal, Error> {
    let symbols = if let Some(symbols) = namespace.get(typ) {
        symbols
    } else {
        return err!(expected_defined_type, location, typ);
    };

    if let Some(Symbol::Type(Type::Terminal(terminal))) = &symbols.get(0).map(|symbol| &symbol.0) {
        Ok(*terminal)
    } else {
        err!(expected_terminal_type, location)
    }
}
