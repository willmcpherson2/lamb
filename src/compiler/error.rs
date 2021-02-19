#[derive(Debug)]
pub struct Error {
    name: &'static str,
    location: Option<usize>,
    message: String,
    compiler_file: &'static str,
    compiler_line: u32,
    compiler_column: u32,
}

impl Error {
    pub fn new(
        name: &'static str,
        location: Option<usize>,
        message: String,
        compiler_file: &'static str,
        compiler_line: u32,
        compiler_column: u32,
    ) -> Self {
        Error {
            name,
            location,
            message,
            compiler_file,
            compiler_line,
            compiler_column,
        }
    }

    #[cfg(test)]
    pub fn name<'a>(&'a self) -> &'a str {
        self.name
    }

    pub fn print(&self, text: &str) {
        let location = if let Some(location) = self.location {
            location
        } else {
            eprint!(
                "Error: {}\n{}:{}:{}\n",
                self.message, self.compiler_file, self.compiler_line, self.compiler_column,
            );
            return;
        };

        enum State {
            Looking,
            Found,
        }
        let mut state = State::Looking;
        let mut line = String::new();
        let mut column = 0;

        for (loc, ch) in text.char_indices() {
            if let State::Looking = state {
                if loc == location {
                    state = State::Found;
                    column = line.len();
                }
                if ch == '\n' {
                    line.clear();
                } else {
                    line.push(ch);
                }
            } else {
                if ch == '\n' {
                    eprint!(
                        "{}\n{}^\nError: {}\n{}:{}:{}\n",
                        line,
                        " ".repeat(column),
                        self.message,
                        self.compiler_file,
                        self.compiler_line,
                        self.compiler_column,
                    );
                    return;
                } else {
                    line.push(ch);
                }
            }
        }

        panic!()
    }
}

macro_rules! err {
    ($($args:tt)*) => {
        Err(error!($($args)*))
    }
}

macro_rules! error {
    () => {
        error_impl!(unimplemented)
    };
    ($name:tt) => {
        error_impl!($name, $name)
    };
    ($name:tt, $($args:tt)*) => {
        error_impl!($name, $name, $($args)*)
    };
}

macro_rules! error_new {
    ($name:tt, $message:expr) => {
        Error::new(
            stringify!($name),
            None,
            $message,
            file!(),
            line!(),
            column!(),
        )
    };
    ($name:tt, $location:expr, $message:expr) => {
        Error::new(
            stringify!($name),
            Some($location),
            $message,
            file!(),
            line!(),
            column!(),
        )
    };
}

macro_rules! error_impl {
    ($name:tt) => {
        error_new!($name, 0, stringify!($name).to_string())
    };

    (expected_paren, $name:tt, $location:expr, $token:expr) => {
        error_new!($name, $location, format!("Unexpected lone token `{}`. You may be missing some parentheses.", $token))
    };

    (expected_func_type_after_name, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Expected function type after this definition name."))
    };

    (expected_func_expr, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Expected function expression after this function type."))
    };

    (expected_func_ret_terminal_type, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Unexpected nesting in function return type."))
    };

    (expected_def, $name:tt, $location:expr, $token:expr) => {
        error_new!($name, $location, format!("Expected definition, got lone token {}.", $token))
    };

    (expected_type, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Expected at least one type inside function type. Try adding `void`."))
    };

    (expected_param, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Expected a parameter consisting of a name and a type."))
    };

    (expected_param_name, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Unexpected nesting, expected a name for a parameter."))
    };

    (expected_param_type, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Unexpected nesting, expected a type for a parameter."))
    };

    (expected_name, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Expected a name to start definition."))
    };

    (expected_func_type, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Expected function type consisting of parameters and a return type."))
    };

    (expected_defined_type, $name:tt, $location:expr, $token:expr) => {
        error_new!($name, $location, format!("No such type `{}`.", $token))
    };

    (expected_defined_symbol, $name:tt, $location:expr, $token:expr) => {
        error_new!($name, $location, format!("Symbol `{}` is undefined.", $token))
    };

    (expected_terminal_type, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Expected terminal type."))
    };

    (unexpected_token, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Unexpected extra token. Function definition should be a name, type and expression."))
    };

    (expected_literal_or_var, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Expected a literal, variable or function call."))
    };

    (expected_func, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Expected a function name in the beginning of function call."))
    };

    (type_mismatch, $name:tt, $location:expr, $expected:expr, $got:expr) => {
        error_new!($name, $location, format!("This type cannot be used. Expected `{:?}`, but got `{:?}`.", $expected, $got))
    };

    (func_type_mismatch, $name:tt, $location:expr, $expected:expr, $got:expr) => {
        error_new!($name, $location, format!("Function call gives wrong type. Expected `{:?}`, but this returns `{:?}`.", $expected, $got))
    };

    (expected_argument, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Missing argument in function call."))
    };

    (unexpected_argument, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Unexpected extra argument in function call."))
    };

    (expected_main, $name:tt) => {
        error_new!($name, format!("Expected `main` function to be defined."))
    };

    (expected_main_type, $name:tt) => {
        error_new!($name, format!("Expected `main` to have type `(i32)` or `(i32 i32)`."))
    };

    (unexpected_multi_main, $name:tt) => {
        error_new!($name, format!("Multiple definitions of function `main`."))
    };

    (no_type_match, $name:tt, $location:expr) => {
        error_new!($name, $location, format!("Functions with this name exist, but none are appropriate in this context."))
    };
}
