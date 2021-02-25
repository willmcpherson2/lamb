use super::*;

macro_rules! ok {
    ($text:literal, $code:literal) => {
        match emit($text) {
            Ok(code) => {
                if code != $code {
                    eprintln!(
                        "\ninput:\n{}\n\nexpected code: \n{}\ngot code:\n{}",
                        $text, $code, code
                    );
                    panic!();
                }
            }
            Err(error) => {
                eprintln!(
                    "\ninput:\n{}\n\nexpected code: \n{}\ngot error:\n{}\n",
                    $text,
                    $code,
                    error.name()
                );
                error.print($text);
                eprintln!();
                panic!();
            }
        }
    };
}

macro_rules! err {
    ($text:literal, $error:literal) => {
        match emit($text) {
            Ok(code) => {
                eprint!(
                    "\ninput:\n{}\n\nexpected error: \n{}\n\ngot code:\n{}\n",
                    $text, $error, code
                );
                panic!();
            }
            Err(error) => {
                if error.name() != $error {
                    eprintln!(
                        "\ninput:\n{}\n\nexpected error: \n{}\n\ngot error:\n{}\n",
                        $text,
                        $error,
                        error.name()
                    );
                    error.print($text);
                    eprintln!();
                    panic!();
                }
            }
        }
    };
}

#[test]
#[allow(clippy::too_many_lines)]
#[allow(clippy::cognitive_complexity)]
fn test() {
    ok!(
        "(f (void) ()) (main (i32) 0)",
        "\
define void @f() {
ret void
}
define i32 @main() {
ret i32 0
}
"
    );

    ok!(
        "(main (i32) 1)",
        "\
define i32 @main() {
ret i32 1
}
"
    );

    ok!(
        "(main ((x i32) i32) x)",
        "\
define i32 @main(i32 %0) {
ret i32 %0
}
"
    );

    ok!(
        "(f ((x i32) (y i32) i32) y) (main (i32) 0)",
        "\
define i32 @f(i32 %0, i32 %1) {
ret i32 %1
}
define i32 @main() {
ret i32 0
}
"
    );

    ok!(
        "(main (i32) (+ 1 2))",
        "\
define i32 @main() {
%1 = add i32 1, 2
ret i32 %1
}
"
    );

    ok!(
        "(f ((x i32) (y i32) i32) (+ x y)) (main (i32) 0)",
        "\
define i32 @f(i32 %0, i32 %1) {
%3 = add i32 %0, %1
ret i32 %3
}
define i32 @main() {
ret i32 0
}
"
    );

    ok!(
        "(f ((x i32) i32) x) (main ((x i32) i32) (f x))",
        "\
define i32 @f(i32 %0) {
ret i32 %0
}
define i32 @main(i32 %0) {
%2 = call i32 @f(i32 %0)
ret i32 %2
}
"
    );

    ok!(
        "(f ((x i32) i32) x) (f (i32) 0) (main (i32) (f))",
        "\
define i32 @f(i32 %0) {
ret i32 %0
}
define i32 @f1() {
ret i32 0
}
define i32 @main() {
%1 = call i32 @f1()
ret i32 %1
}
"
    );

    ok!(
        "(f (i32) 0) (f (i32) 1) (main (i32) (f))",
        "\
define i32 @f() {
ret i32 0
}
define i32 @f1() {
ret i32 1
}
define i32 @main() {
%1 = call i32 @f1()
ret i32 %1
}
"
    );

    ok!(
        "(f ((x i32) i32) x) (f ((x i32) i32) x) (main (i32) (f 1))",
        "\
define i32 @f(i32 %0) {
ret i32 %0
}
define i32 @f1(i32 %0) {
ret i32 %0
}
define i32 @main() {
%1 = call i32 @f1(i32 1)
ret i32 %1
}
"
    );

    ok!(
        "(f ((x i32) (y i32) i32) (* x y)) (main (i32) (f 1 2))",
        "\
define i32 @f(i32 %0, i32 %1) {
%3 = mul i32 %0, %1
ret i32 %3
}
define i32 @main() {
%1 = call i32 @f(i32 1, i32 2)
ret i32 %1
}
"
    );

    ok!(
        "(f (f32) (+ 1.0 2.0)) (main (i32) 0)",
        "\
define float @f() {
%1 = fadd float 1.0, 2.0
ret float %1
}
define i32 @main() {
ret i32 0
}
"
    );

    ok!(
        "(main (i32) (~ 1))",
        "\
define i32 @main() {
%1 = xor i32 1, -1
ret i32 %1
}
"
    );

    ok!(
        "(main ((x i32) i32) (/ x x))",
        "\
define i32 @main(i32 %0) {
%2 = sdiv i32 %0, %0
ret i32 %2
}
"
    );

    ok!(
        "(main ((x i32) i32) (<< x 3))",
        "\
define i32 @main(i32 %0) {
%2 = shl i32 %0, 3
ret i32 %2
}
"
    );

    ok!(
        "(f (i8) (+ 1 2)) (main (i32) 0)",
        "\
define i8 @f() {
%1 = add i8 1, 2
ret i8 %1
}
define i32 @main() {
ret i32 0
}
"
    );

    ok!(
        "(main (i32 i32) 1)",
        "\
define i32 @main(i32 %0) {
ret i32 1
}
"
    );

    ok!(
        "(f (i32 i32 i32) (+ 1 2)) (main (i32 i32) (f 1 2))",
        "\
define i32 @f(i32 %0, i32 %1) {
%3 = add i32 1, 2
ret i32 %3
}
define i32 @main(i32 %0) {
%2 = call i32 @f(i32 1, i32 2)
ret i32 %2
}
"
    );

    ok!(
        "(f (i32 (y i32) i32) y) (f (i32) (f 1 2)) (main (i32) (f))",
        "\
define i32 @f(i32 %0, i32 %1) {
ret i32 %1
}
define i32 @f1() {
%1 = call i32 @f(i32 1, i32 2)
ret i32 %1
}
define i32 @main() {
%1 = call i32 @f1()
ret i32 %1
}
"
    );

    ok!(
        "(f (void) ()) (g (void) (f)) (main (i32) 0)",
        "\
define void @f() {
ret void
}
define void @g() {
call void @f()
ret void
}
define i32 @main() {
ret i32 0
}
"
    );

    ok!(
        "(f (i32) 0) (f (i32) (f)) (main (i32) (f))",
        "\
define i32 @f() {
ret i32 0
}
define i32 @f1() {
%1 = call i32 @f()
ret i32 %1
}
define i32 @main() {
%1 = call i32 @f1()
ret i32 %1
}
"
    );

    err!("a", "expected_def");

    err!("(main (void) () ()) ((x i32", "unexpected_token");

    err!("(main void ())", "expected_func_type");

    err!("(main)", "expected_func_type_after_name");

    err!("(main (i32))", "expected_func_expr");

    err!("(main () ())", "expected_type");

    err!("(main (()) ())", "expected_func_ret_terminal_type");

    err!("(main ((i32) i32) 1)", "expected_param");

    err!("(main ((() i32) i32) 1)", "expected_param_name");

    err!("(main ((x ()) i32) 1)", "expected_param_type");

    err!("(main ((x a32) i32) 1)", "expected_defined_type");

    err!("(main ((x i32) a32) 1)", "expected_defined_type");

    err!("(f (i32) 1) (main ((x f) i32) 1)", "expected_terminal_type");

    err!("(f (i32) 1) (main ((x i32) f) 1)", "expected_terminal_type");

    err!(
        "(f (i32) 1) (main ((x i32) i32) f)",
        "expected_literal_or_var"
    );

    err!("(main ((x i32) i32) true)", "type_mismatch");

    err!("(f (i32) 1) (main (i32) ((f)))", "expected_func");

    err!("(main (i32) (f))", "expected_defined_symbol");

    err!("(main (i32) (i32))", "expected_func");

    err!("(f (f32) 1.0) (main (i32) (f))", "func_type_mismatch");

    err!("(f (i32) 1) (main (i32) (f 1))", "unexpected_argument");

    err!(
        "(f ((x i32) i32) 1) (main (i32) (f 1 2))",
        "unexpected_argument"
    );

    err!("(f ((x i32) i32) 1) (main (i32) (f))", "expected_argument");

    err!(
        "(f ((x i32) (y i32) i32) 1) (main (i32) (f 1))",
        "expected_argument"
    );

    err!("(f (i32) 0)", "expected_main");

    err!("(main (void) 0)", "expected_main_type");

    err!("(main ((x i32) void) ())", "expected_main_type");

    err!(
        "(f (i32) 0) (f (i32) 1) (main (i32) (f 1))",
        "no_type_match"
    );

    err!(
        "(f (f32) 0.0) (f (f32) 1.0) (main (i32) (f))",
        "no_type_match"
    );

    err!(
        "(f ((x i32) i32) 0) (f (i32) 1) (main (i32) (f 1 2))",
        "no_type_match"
    );

    err!("(main (i32) (^ 1 2 3))", "no_type_match");

    err!("(main ((x i32) i32) (&& x true))", "func_type_mismatch");

    err!("(main ((x i32) i32) (&& x 1))", "func_type_mismatch");

    err!("(main ((x i32) i32) (<< x true))", "no_type_match");

    err!("(main ((x i32) i32) (|| x 1))", "func_type_mismatch");
}
