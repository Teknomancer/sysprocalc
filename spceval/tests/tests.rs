use spceval::{self, Number, ExprError, ExprErrorKind};

#[test]
fn valid_exprs() {
    // Don't bother testing different radices here. Those are already covered by unit tests.
    // Here we should focus on expression constructs rather than validating parsing of numbers.
    // Focus on testing:
    //   - Operator preceedence.
    //   - Parenthesis and related priority.
    //   - Functions.
    //   - Maybe whitespace in odd places.
    // Make each expression test meaningful and try not to have redundant tests.
    let expr_results = vec![
        ("2+2", Number{ integer: 4, float: 4.0 }),
        ("+55.5", Number{ integer: 55, float: 55.5 }),
        ("-4", Number{ integer: 0xfffffffffffffffc, float: -4.0 }),
        ("-4 -4", Number{ integer: 0xfffffffffffffff8, float: -8.0 }),
        ("+8 +8", Number{ integer: 16, float: 16.0 }),
        ("+8 + -2", Number{ integer: 6, float: 6.0 }),
        ("-8 - -2", Number{ integer: 0xfffffffffffffffa, float: -6.0 }),
        ("(1234)", Number{ integer: 1234, float: 1234.0 }),
        ("(2+2)", Number{ integer: 4, float: 4.0 }),
        ("1+2*3", Number{ integer: 7, float: 7.0 }),
        ("(1+2)*3", Number{ integer: 9, float: 9.0 }),
        ("(1+2)*(5-1)", Number{ integer: 12, float: 12.0 }),
        ("0xf << 1", Number{ integer: 0x1e, float: 30.0 }),
        ("((0x128)) + 0n111", Number{ integer: 303, float: 303.0 }),
        ("1*4+(0n1+0xf)", Number{ integer: 20, float: 20.0 }),
    ];

    for expr_res in expr_results {
        let res_parse = spceval::parse(&expr_res.0);
        assert!(res_parse.is_ok());

        let mut expr_ctx = res_parse.unwrap();
        let res_eval = spceval::evaluate(&mut expr_ctx);
        assert!(res_eval.is_ok());

        let res_expr = res_eval.unwrap();
        match res_expr {
            spceval::ExprResult::Number(n) => {
                assert_eq!(expr_res.1.integer, n.integer);
                assert_eq!(expr_res.1.float, n.float);
            }
            _ => (),
        }
    }
}

#[test]
fn invalid_exprs() {
    use ExprErrorKind::*;
    let expr_results = vec![
        ("", ExprError { idx_expr: 0, kind: EmptyExpr, message: "".to_string() }),
        ("()", ExprError { idx_expr: 0, kind: EmptyExpr, message: "".to_string() }),
        ("2 +", ExprError { idx_expr: 0, kind: InvalidParamCount, message: "".to_string() }),
        ("- -2", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        ("+ +2", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        (",2", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        ("(", ExprError { idx_expr: 0, kind: MismatchParenthesis, message: "".to_string() }),
        (")", ExprError { idx_expr: 0, kind: MismatchParenthesis, message: "".to_string() }),
        (",", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        ("(,", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        (",)", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        ("(5,", ExprError { idx_expr: 0, kind: MissingFunction, message: "".to_string() }),
        ("(2 +", ExprError { idx_expr: 0, kind: MismatchParenthesis, message: "".to_string() }),
        ("2 + 5)", ExprError { idx_expr: 0, kind: MismatchParenthesis, message: "".to_string() }),
        ("++", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        ("--", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        (",5,", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        ("0,", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        ("(),", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        ("(0),", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        ("5+()", ExprError { idx_expr: 0, kind: InvalidParamCount, message: "".to_string() }),
        ("-()", ExprError { idx_expr: 0, kind: InvalidParamCount, message: "".to_string() }),
        ("+()", ExprError { idx_expr: 0, kind: InvalidParamCount, message: "".to_string() }),
        (",(),", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        ("(),()", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
        ("(1),()", ExprError { idx_expr: 0, kind: MissingParenthesis, message: "".to_string() }),
        ("(1),(2)", ExprError { idx_expr: 0, kind: MissingParenthesis, message: "".to_string() }),
        ("(1),(2),", ExprError { idx_expr: 0, kind: MissingParenthesis, message: "".to_string() }),
        ("(1)(2),", ExprError { idx_expr: 0, kind: MissingOperatorOrFunction, message: "".to_string() }),
        ("5(2),", ExprError { idx_expr: 0, kind: MissingOperatorOrFunction, message: "".to_string() }),
        ("5,(2),", ExprError { idx_expr: 0, kind: MissingParenthesis, message: "".to_string() }),
        // The below expressions need fixing in the parser/evaluation phase in the library!
        // They are parsing/evaluation bugs!
        //("(1).5", ExprError { idx_expr: 0, kind: InvalidExpr, message: "".to_string() }),
    ];
    for expr_res in expr_results {
        let res_parse = spceval::parse(&expr_res.0);
        if (res_parse.is_ok()) {
            let mut expr_ctx = res_parse.unwrap();
            let res_eval = spceval::evaluate(&mut expr_ctx);
            assert!(res_eval.is_err());
            assert_eq!(expr_res.1.kind, res_eval.err().unwrap().kind, "{}", expr_res.0);
        } else {
            assert_eq!(expr_res.1.kind, res_parse.err().unwrap().kind, "{}", expr_res.0);
        }
    }
}

