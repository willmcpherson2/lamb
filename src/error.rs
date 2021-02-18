#[derive(Debug)]
pub struct Error {
    name: &'static str,
    message: String,
    lamb_file: &'static str,
    lamb_line: u32,
    lamb_column: u32,
}

impl Error {
    pub fn new(
        name: &'static str,
        message: String,
        lamb_file: &'static str,
        lamb_line: u32,
        lamb_column: u32,
    ) -> Self {
        Error {
            name,
            message,
            lamb_file,
            lamb_line,
            lamb_column,
        }
    }

    pub fn print(&self) {
        eprint!(
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
        Error::new(stringify!($name), $message, file!(), line!(), column!())
    };
}

macro_rules! error_impl {
    ($name:tt) => {
        error_new!($name, stringify!($name).to_string())
    };

    (expected_filename, $name:tt) => {
        error_new!($name, format!("Expected filename."))
    };

    (file_error, $name:tt, $filename:expr) => {
        error_new!(
            $name,
            format!("Could not read file `{}`. Does the file exist?", $filename)
        )
    };

    (clang_spawn_failed, $name:tt) => {
        error_new!(
            $name,
            format!("Could not spawn clang. Is it installed? Is it in your $PATH?")
        )
    };

    (clang_stdin_failed, $name:tt) => {
        error_new!($name, format!("Could not open stdin to clang."))
    };

    (clang_write_failed, $name:tt) => {
        error_new!($name, format!("Could not write to clang via stdin."))
    };

    (clang_output_failed, $name:tt) => {
        error_new!($name, format!("Could not read output from clang."))
    };

    (clang_status_failed, $name:tt) => {
        error_new!($name, format!("Could not read exit status from clang."))
    };

    (clang_non_zero, $name:tt, $code:expr) => {
        error_new!($name, format!("clang return with exit code {}.", $code))
    };
}
