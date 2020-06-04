use spceval::{self, Number, ExprError, ExprErrorKind};

#[test]
fn valid_exprs() {
    // These are valid expressions and must produce the right results.
    // Don't bother testing different numeric radices here, those are already covered
    // by unit tests. Here we should focus on expression constructs rather than
    // validating parsing of numbers.
    // Focus on testing:
    //   - Operator preceedence.
    //   - Parenthesis and related priority.
    //   - Functions.
    //   - Maybe whitespace in odd places.
    // Make each expression test meaningful and try not to have redundant tests.
    // TODO: Try to split this into logical categories like unary, binary,
    // paranthesis, operator priority etc.
    let expr_results = vec![
        ("2+2", Number { integer: 4, float: 4.0 }),
        ("+55.5", Number { integer: 55, float: 55.5 }),
        ("-4", Number { integer: 0xfffffffffffffffc, float: -4.0 }),
        ("-4 -4", Number { integer: 0xfffffffffffffff8, float: -8.0 }),
        ("+8 +8", Number { integer: 16, float: 16.0 }),
        ("+8 + -2", Number { integer: 6, float: 6.0 }),
        ("-8 - -2", Number { integer: 0xfffffffffffffffa, float: -6.0 }),
        ("(0)", Number { integer: 0, float: 0.0 }),
        ("(45)", Number { integer: 45, float: 45.0 }),
        ("(-5)", Number { integer: 0xfffffffffffffffb, float: -5.0 }),
        ("(((1220)))", Number { integer: 1220, float: 1220.0 }),
        ("(-.5)", Number { integer: 0, float: -0.5 }),
        ("(1234)", Number { integer: 1234, float: 1234.0 }),
        ("(2+2)", Number { integer: 4, float: 4.0 }),
        ("1+2*3", Number { integer: 7, float: 7.0 }),
        ("(1+2)*3", Number { integer: 9, float: 9.0 }),
        ("(1+2)*(5-1)", Number { integer: 12, float: 12.0 }),
        ("0xf << 1", Number { integer: 0x1e, float: 30.0 }),
        ("((0x128)) + 0n111", Number { integer: 303, float: 303.0 }),
        ("1*4+(0n1+0xf)", Number { integer: 20, float: 20.0 }),
        (".5*0", Number { integer: 0, float: 0.0 }),
        ("5/(5/(5/(5)))", Number { integer: 1, float: 1.0 }),
        ("212 + (1 * (3 - (4 * 5)))", Number { integer: 195, float: 195.0 }),
        ("0*5", Number { integer: 0, float: 0.0 }),
        ("0/5", Number { integer: 0, float: 0.0 }),
    ];

    for expr_res in expr_results {
        let res_parse = spceval::parse(&expr_res.0);
        assert!(res_parse.is_ok(), "{}", expr_res.0);

        let mut expr_ctx = res_parse.unwrap();
        let res_eval = spceval::evaluate(&mut expr_ctx);
        assert!(res_eval.is_ok(), "{}", expr_res.0);

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
fn valid_exprs_eval_fail() {
    // These are expressions that are syntactically valid but guaranteed to fail during
    // evaluation. E.g "1/0" is perfectly valid syntax but fails due to division by zero.
    // These must never produce errors during the parsing phase.
    use ExprErrorKind::*;
    let expr_results = vec![
        ("1/0", ExprError { idx_expr: 0, kind: FailedEvaluation, message: String::new() }),
    ];
    for expr_res in expr_results {
        let res_parse = spceval::parse(&expr_res.0);
        assert!(res_parse.is_ok(), "{}", expr_res.0);
        let mut expr_ctx = res_parse.unwrap();
        let res_eval = spceval::evaluate(&mut expr_ctx);
        assert!(res_eval.is_err(), "{}", expr_res.0);
        assert_eq!(expr_res.1.kind, res_eval.err().unwrap().kind, "{}", expr_res.0);
    }
}

#[test]
fn invalid_exprs() {
    // These are expressions that MUST produce errors in either the parse or evaluation
    // phase. As long as they produce the required errors it's fine. Some expressions
    // like "2 +" fail during evaluation due to the way we parse operators but others
    // like ",5" will fail during parsing.
    use ExprErrorKind::*;
    let expr_results = vec![
        ("", ExprError { idx_expr: 0, kind: EmptyExpr, message: String::new() }),
        ("()", ExprError { idx_expr: 0, kind: EmptyExpr, message: String::new() }),
        ("2 +", ExprError { idx_expr: 0, kind: InvalidParamCount, message: String::new() }),
        ("- -2", ExprError { idx_expr: 0, kind: MissingOperand, message: String::new() }),
        ("+ +2", ExprError { idx_expr: 0, kind: MissingOperand, message: String::new() }),
        (",2", ExprError { idx_expr: 0, kind: InvalidExpr, message: String::new() }),
        ("(", ExprError { idx_expr: 0, kind: MismatchParenthesis, message: String::new() }),
        (")", ExprError { idx_expr: 0, kind: MismatchParenthesis, message: String::new() }),
        (",", ExprError { idx_expr: 0, kind: InvalidExpr, message: String::new() }),
        ("(,", ExprError { idx_expr: 0, kind: InvalidExpr, message: String::new() }),
        (",)", ExprError { idx_expr: 0, kind: InvalidExpr, message: String::new() }),
        ("(5,", ExprError { idx_expr: 0, kind: MissingFunction, message: String::new() }),
        ("(2 +", ExprError { idx_expr: 0, kind: MismatchParenthesis, message: String::new() }),
        ("2 + 5)", ExprError { idx_expr: 0, kind: MismatchParenthesis, message: String::new() }),
        ("++", ExprError { idx_expr: 0, kind: MissingOperand, message: String::new() }),
        ("--", ExprError { idx_expr: 0, kind: MissingOperand, message: String::new() }),
        ("0,", ExprError { idx_expr: 0, kind: MissingParenthesis, message: String::new() }),
        ("(),", ExprError { idx_expr: 0, kind: MissingParenthesis, message: String::new() }),
        ("(0),", ExprError { idx_expr: 0, kind: MissingParenthesis, message: String::new() }),
        ("5+()", ExprError { idx_expr: 0, kind: InvalidParamCount, message: String::new() }),
        ("-()", ExprError { idx_expr: 0, kind: InvalidParamCount, message: String::new() }),
        ("+()", ExprError { idx_expr: 0, kind: InvalidParamCount, message: String::new() }),
        (",(),", ExprError { idx_expr: 0, kind: InvalidExpr, message: String::new() }),
        ("(),()", ExprError { idx_expr: 0, kind: MissingParenthesis, message: String::new() }),
        ("(1),()", ExprError { idx_expr: 0, kind: MissingParenthesis, message: String::new() }),
        ("(1),(2)", ExprError { idx_expr: 0, kind: MissingParenthesis, message: String::new() }),
        ("(1),(2),", ExprError { idx_expr: 0, kind: MissingParenthesis, message: String::new() }),
        ("(1)(2),", ExprError { idx_expr: 0, kind: MissingOperatorOrFunction, message: String::new() }),
        ("5(2),", ExprError { idx_expr: 0, kind: MissingOperatorOrFunction, message: String::new() }),
        ("5,(2),", ExprError { idx_expr: 0, kind: MissingParenthesis, message: String::new() }),
        ("(<<2", ExprError { idx_expr: 0, kind: InvalidExpr, message: String::new() }),
        ("(1)2", ExprError { idx_expr: 0, kind: MissingOperatorOrFunction, message: String::new() }),
        ("(1).5", ExprError { idx_expr: 0, kind: MissingOperatorOrFunction, message: String::new() }),
        ("2 << << 4", ExprError { idx_expr: 0, kind: InvalidExpr, message: String::new() }),
        ("2<<<<4", ExprError { idx_expr: 0, kind: InvalidExpr, message: String::new() }),
        ("2+ + +4", ExprError { idx_expr: 0, kind: MissingOperand, message: String::new() }),
        ("2+++4", ExprError { idx_expr: 0, kind: MissingOperand, message: String::new() }),
        (".+2", ExprError { idx_expr: 0, kind: InvalidExpr, message: String::new() }),
        (".-2", ExprError { idx_expr: 0, kind: InvalidExpr, message: String::new() }),
        ("(.5", ExprError { idx_expr: 0, kind: MismatchParenthesis, message: String::new() }),
        (").f", ExprError { idx_expr: 0, kind: MismatchParenthesis, message: String::new() }),
        ("(-1).5", ExprError { idx_expr: 0, kind: MissingOperatorOrFunction, message: String::new() }),
    ];
    for expr_res in expr_results {
        let res_parse = spceval::parse(&expr_res.0);
        if res_parse.is_ok() {
            let mut expr_ctx = res_parse.unwrap();
            let res_eval = spceval::evaluate(&mut expr_ctx);
            assert!(res_eval.is_err(), "{}", expr_res.0);
            assert_eq!(expr_res.1.kind, res_eval.err().unwrap().kind, "{}", expr_res.0);
        } else {
            assert_eq!(expr_res.1.kind, res_parse.err().unwrap().kind, "{}", expr_res.0);
        }
    }
}

