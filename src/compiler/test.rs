use super::*;

macro_rules! ok {
    ($text:literal, $code:literal) => {
        if let Ok(code) = emit($text) {
            assert_eq!(code, $code);
        } else {
            unreachable!("expected Ok, got Err");
        }
    };
}

macro_rules! err {
    ($text:literal, $error:literal) => {
        if let Err(error) = emit($text) {
            assert_eq!(error.name(), $error);
        } else {
            unreachable!("expected Err, got Ok");
        }
    };
}

#[test]
fn test() {
    ok!(
        "(main (void) ())",
        "\
define void @main() {
ret void
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
        "(main ((x i32) (y i32) i32) y)",
        "\
define i32 @main(i32 %0, i32 %1) {
ret i32 %1
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
        "(main ((x i32) (y i32) i32) (+ x y))",
        "\
define i32 @main(i32 %0, i32 %1) {
%3 = add i32 %0, %1
ret i32 %3
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

    err!("a", "expected_def");

    err!("(main (void) () ()) ((x i32", "unexpected_token");

    err!("(main void ())", "expected_func_type");

    err!("(main)", "expected_func_type_after_name");

    err!("(main (i32))", "expected_func_expr");

    err!("(main () ())", "expected_type");

    err!("(main (()) ())", "expected_func_ret_terminal_type");

    err!("(main (i32 i32) 1)", "expected_param");

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
}
