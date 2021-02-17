#[derive(Debug)]
pub struct Error {
    location: usize,
    message: String,
    compiler_file: &'static str,
    compiler_line: u32,
    compiler_column: u32,
}

impl Error {
    pub fn new(
        location: usize,
        message: String,
        compiler_file: &'static str,
        compiler_line: u32,
        compiler_column: u32,
    ) -> Self {
        Error {
            location,
            message,
            compiler_file,
            compiler_line,
            compiler_column,
        }
    }
}

impl Error {
    pub fn print(&self, text: &str) {
        enum State {
            Looking,
            Found,
        }
        let mut state = State::Looking;
        let mut line = String::new();
        let mut column = 0;

        for (loc, ch) in text.char_indices() {
            if let State::Looking = state {
                if loc == self.location {
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

macro_rules! error_new {
    ($location:expr, $message:expr) => {
        Error::new($location, $message, file!(), line!(), column!())
    };
}

macro_rules! error {
    () => {{
        let message = format!("unimplemented");
        error_new!(0, message)
    }};

    ("expected paren", $location:expr, $token:expr) => {{
        error_new!($location, format!("Unexpected lone token `{}`. You may be missing some parentheses.", $token))
    }};

    ("expected func type after name", $location:expr) => {{
        error_new!($location, format!("Expected function type after this definition name."))
    }};

    ("expected func expr", $location:expr) => {{
        error_new!($location, format!("Expected function expression after this function type."))
    }};

    ("expected func ret terminal type", $location:expr) => {{
        error_new!($location, format!("Unexpected nesting in function return type."))
    }};

    ("expected def", $location:expr, $token:expr) => {{
        error_new!($location, format!("Expected definition, got lone token {}.", $token))
    }};

    ("expected type", $location:expr) => {{
        error_new!($location, format!("Expected at least one type inside function type. Try adding `void`."))
    }};

    ("expected param", $location:expr) => {{
        error_new!($location, format!("Expected a parameter consisting of a name and a type."))
    }};

    ("expected param name", $location:expr) => {{
        error_new!($location, format!("Unexpected nesting, expected a name for a parameter."))
    }};

    ("expected param type", $location:expr) => {{
        error_new!($location, format!("Unexpected nesting, expected a type for a parameter."))
    }};

    ("expected name", $location:expr) => {{
        error_new!($location, format!("Expected a name to start definition."))
    }};

    ("expected func type", $location:expr) => {{
        error_new!($location, format!("Expected function type consisting of parameters and a return type."))
    }};

    ("expected defined type", $location:expr, $token:expr) => {{
        error_new!($location, format!("No such type `{}`.", $token))
    }};

    ("expected defined symbol", $location:expr, $token:expr) => {{
        error_new!($location, format!("Symbol `{}` is undefined.", $token))
    }};

    ("expected terminal type", $location:expr) => {{
        error_new!($location, format!("Expected terminal type."))
    }};

    ("unexpected token", $location:expr) => {{
        error_new!($location, format!("Unexpected extra token. Function definition should be a name, type and expression."))
    }};

    ("expected literal or var", $location:expr) => {{
        error_new!($location, format!("Expected a literal, variable or function call."))
    }};

    ("expected func", $location:expr) => {{
        error_new!($location, format!("Expected a function name in the beginning of function call."))
    }};

    ("type mismatch", $location:expr, $expected:expr, $got:expr) => {{
        error_new!($location, format!("This type cannot be used. Expected `{:?}`, but got `{:?}`.", $expected, $got))
    }};

    ("func type mismatch", $location:expr, $expected:expr, $got:expr) => {{
        error_new!($location, format!("Function call gives wrong type. Expected `{:?}`, but this returns `{:?}`.", $expected, $got))
    }};

    ("expected argument", $location:expr) => {{
        error_new!($location, format!("Missing argument in function call."))
    }};

    ("unexpected argument", $location:expr) => {{
        error_new!($location, format!("Unexpected extra argument in function call."))
    }};
}
