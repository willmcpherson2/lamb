use std::fmt;

#[derive(Debug)]
pub struct Error {
    message: String,
    lamb_file: &'static str,
    lamb_line: u32,
    lamb_column: u32,
}

impl Error {
    pub fn new(message: String, lamb_file: &'static str, lamb_line: u32, lamb_column: u32) -> Self {
        Error {
            message,
            lamb_file,
            lamb_line,
            lamb_column,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error: {}\n{}:{}:{}\n",
            self.message, self.lamb_file, self.lamb_line, self.lamb_column
        )
    }
}

macro_rules! err {
    ($($args:tt)*) => {
        Err(error!($($args)*))
    }
}

macro_rules! error_new {
    ($message:expr) => {
        Error::new($message, file!(), line!(), column!())
    };
}

macro_rules! error {
    () => {{
        error_new!(format!("unimplemented"))
    }};

    ("expected arg 1") => {{
        error_new!(format!("Expected at least 1 command line argument (program name). Maybe something is wrong with your shell?"))
    }};

    ("expected filename") => {{
        error_new!(format!("Expected filename."))
    }};

    ("file error", $filename:expr) => {{
        error_new!(format!("Could not read file `{}`. Does the file exist?", $filename))
    }};

    ("clang spawn failed") => {{
        error_new!(format!("Could not spawn clang. Is it installed? Is it in your $PATH?"))
    }};

    ("clang stdin failed") => {{
        error_new!(format!("Could not open stdin to clang."))
    }};

    ("clang write failed") => {{
        error_new!(format!("Could not write to clang via stdin."))
    }};

    ("clang output failed") => {{
        error_new!(format!("Could not read output from clang."))
    }};

    ("clang status failed") => {{
        error_new!(format!("Could not read exit status from clang."))
    }};

    ("clang non-zero", $code:expr) => {{
        error_new!(format!("clang return with exit code {}.", $code))
    }};
}
