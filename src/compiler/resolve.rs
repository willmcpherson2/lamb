use super::error::Error;
use super::namespace::Namespace;
use super::parse::Program;
use super::symbol::Func;
use super::symbol::Symbol;
use super::symbol::Type;

pub fn resolve(program: Program, mut namespace: Namespace) -> Result<(Program, Namespace), Error> {
    for def in &program.defs {
        let name = def.name.0.clone();
        let mut def_namespace = Namespace::new();

        let mut params = Vec::new();
        for param in &def.func.params {
            let param_name = param.name.0.clone();

            let param_type = if let Some(symbol) = namespace.get(&param.typ.0) {
                if let Symbol::Type(Type::Terminal(terminal)) = symbol {
                    *terminal
                } else {
                    return err!("expected terminal type", param.typ.1);
                }
            } else {
                return err!("expected defined type", param.typ.1, &param.typ.0);
            };
            params.push(param_type);

            let symbol = Symbol::Var(Type::Terminal(param_type));
            def_namespace.insert(param_name, symbol);
        }

        let ret = if let Some(symbol) = namespace.get(&def.func.ret.0) {
            if let Symbol::Type(Type::Terminal(terminal)) = symbol {
                *terminal
            } else {
                return err!("expected terminal type", def.func.ret.1);
            }
        } else {
            return err!("expected defined type", def.func.ret.1, &def.func.ret.0);
        };

        let symbol = Symbol::Var(Type::Func(Func { params, ret }));
        namespace.insert_with_namespace(name, symbol, def_namespace);
    }

    Ok((program, namespace))
}
