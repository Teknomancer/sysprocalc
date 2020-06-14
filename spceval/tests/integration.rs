use spceval::{self, Number, ExprError, ExprErrorKind};

#[inline(always)]
fn test_valid_expr(str_expr: &str, res_num: &Number) {
    let res_parse = spceval::parse(str_expr);
    assert!(res_parse.is_ok(), "{}", str_expr);

    let mut expr_ctx = res_parse.unwrap();
    let res_eval = spceval::evaluate(&mut expr_ctx);
    assert!(res_eval.is_ok(), "{}", str_expr);

    let res_expr = res_eval.unwrap();
    match res_expr {
        spceval::ExprResult::Number(n) => {
            assert_eq!(res_num.integer, n.integer, "{}", str_expr);
            assert_eq!(res_num.float, n.float, "{}", str_expr);
        }
        _ => (),
    }
}

#[test]
fn valid_exprs_unary_opers() {
    let expr_results = vec![
        // Unary minus
        ("-0", Number { integer: 0, float: 0.0 }),
        ("-1", Number { integer: -1i64 as u64, float: -1.0 }),
        ("-120", Number { integer: -120i64 as u64, float: -120.0 }),
        ("-(0)", Number { integer: 0, float: 0.0 }),
        ("-(1)", Number { integer: -1i64 as u64, float: -1.0 }),
        ("-(120)", Number { integer: -120i64 as u64, float: -120.0 }),
        // Unary plus
        ("+0", Number { integer: 0, float: 0.0 }),
        ("+1", Number { integer: 1, float: 1.0 }),
        ("+(0)", Number { integer: 0, float: 0.0 }),
        ("+(1)", Number { integer: 1, float: 1.0 }),
        ("+(120)", Number { integer: 120, float: 120.0 }),
        // Logical NOT.
        ("!0", Number { integer: 1, float: 1.0 }),
        ("!1", Number { integer: 0, float: 0.0 }),
        ("!2", Number { integer: 0, float: 0.0 }),
        ("!123", Number { integer: 0, float: 0.0 }),
        ("!(0)", Number { integer: 1, float: 1.0 }),
        ("!(1)", Number { integer: 0, float: 0.0 }),
        ("!(123)", Number { integer: 0, float: 0.0 }),
        ("!(-1)", Number { integer: 0, float: 0.0 }),
        ("!(-2)", Number { integer: 0, float: 0.0 }),
        ("!(-123)", Number { integer: 0, float: 0.0 }),
        // Bitwise NOT.
        ("~0", Number { integer: !0u64, float: !0u64 as f64 }),
        ("~1", Number { integer: !1u64, float: !1u64 as f64 }),
        ("~2", Number { integer: !2u64, float: !2u64 as f64 }),
        ("~145", Number { integer: !145u64, float: !145u64 as f64 }),
        ("~(0)", Number { integer: !0u64, float: !0u64 as f64 }),
        ("~(1)", Number { integer: !1u64, float: !1u64 as f64 }),
        ("~(2)", Number { integer: !2u64, float: !2u64 as f64 }),
        ("~(145)", Number { integer: !145u64, float: !145u64 as f64 }),
        ("~(-1)", Number { integer: !-1i64 as u64, float: !-1i64 as u64 as f64 }),
        ("~(-2)", Number { integer: !-2i64 as u64, float: !-2i64 as u64 as f64 }),
        ("~(-145)", Number { integer: !-145i64 as u64, float: !-145i64 as u64 as f64 }),
    ];
    for expr_res in expr_results {
        test_valid_expr(&expr_res.0, &expr_res.1);
    }
}

#[test]
fn valid_exprs_binary_opers() {
    let expr_results = vec![
        //
        // Add
        //
        ("0+0", Number { integer: 0, float: 0.0 }),
        ("0+1", Number { integer: 1, float: 1.0 }),
        ("1+1", Number { integer: 2, float: 2.0 }),
        ("2+2", Number { integer: 4, float: 4.0 }),
        ("132+132", Number { integer: 132+132, float: (132+132) as f64 }),
        ("0xf0f0f0f0+0", Number { integer: 0xf0f0f0f0, float: 0xf0f0f0f0u64 as f64 }),
        ("0xf0f0f0f0+1", Number { integer: 0xf0f0f0f0u64.wrapping_add(1), float: (0xf0f0f0f0u64) as f64 + 1.0 }),
        ("0xf0f0f0f0+0xf0f0f0f0",
            Number { integer: 0xf0f0f0f0u64.wrapping_add(0xf0f0f0f0),
                     float: (0xf0f0f0f0u64.wrapping_add(0xf0f0f0f0)) as f64 }),
        ("0xf0f0f0f0f0f0f0f0+0",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_add(0),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 }),
        ("0xf0f0f0f0f0f0f0f0+1",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_add(1),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 + 1.0 }),
        ("0xf0f0f0f0f0f0f0f0+0xf0f0f0f0f0f0f0f0",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_add(0xf0f0f0f0f0f0f0f0),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 + 0xf0f0f0f0f0f0f0f0u64 as f64 }),
        ("0xffffffff+0", Number { integer: 0xffffffff, float: 0xffffffffu64 as f64 }),
        ("0xffffffff+1", Number { integer: 0xffffffff+1, float: (0xffffffffu64+1u64) as f64 }),
        ("0xffffffff+0xffffffff",
            Number { integer: 0xffffffffu64.wrapping_add(0xffffffff),
                     float: (0xffffffffu64.wrapping_add(0xffffffff)) as f64 }),
        ("0xffffffffffffffff+0",
            Number { integer: 0xffffffffffffffffu64.wrapping_add(0),
                     float: 0xffffffffffffffffu64 as f64 }),
        ("0xffffffffffffffff+1",
            Number { integer: 0xffffffffffffffffu64.wrapping_add(1),
                     float: 0xffffffffffffffffu64 as f64 + 1.0 }),
        ("0xffffffffffffffff+0xffffffffffffffff",
            Number { integer: 0xffffffffffffffffu64.wrapping_add(0xffffffffffffffff),
                     float: 0xffffffffffffffffu64 as f64 + 0xffffffffffffffffu64 as f64 }),

        //
        // Subtract
        //
        ("0-0", Number { integer: 0, float: 0.0 }),
        ("0-1", Number { integer: 0u64.wrapping_sub(1), float: -1.0 }),
        ("1-1", Number { integer: 0, float: 0.0 }),
        ("12-2", Number { integer: 10, float: 10.0 }),
        ("132-100", Number { integer: 132-100, float: (132-100) as f64 }),

        ("0xf0f0f0f0-0", Number { integer: 0xf0f0f0f0, float: 0xf0f0f0f0u64 as f64 }),
        ("0xf0f0f0f0-1", Number { integer: 0xf0f0f0f0u64.wrapping_sub(1), float: 0xf0f0f0f0u64 as f64 - 1.0 }),
        ("0xf0f0f0f0-0xf0f0f0f0",
            Number { integer: 0xf0f0f0f0u64.wrapping_sub(0xf0f0f0f0),
                     float: 0xf0f0f0f0u64 as f64 - 0xf0f0f0f0u64 as f64 }),
        ("0xf0f0f0f0f0f0f0f0-0",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_sub(0),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 }),
        ("0xf0f0f0f0f0f0f0f0-1",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_sub(1),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 - 1.0 }),
        ("0xf0f0f0f0f0f0f0f0-0xf0f0f0f0f0f0f0f0",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_sub(0xf0f0f0f0f0f0f0f0),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 - 0xf0f0f0f0f0f0f0f0u64 as f64 }),
        ("0xffffffff-0", Number { integer: 0xffffffff, float: 0xffffffffu64 as f64 }),
        ("0xffffffff-1", Number { integer: 0xffffffff-1, float: 0xffffffffu64 as f64 - 1.0 }),
        ("0xffffffff-0xffffffff",
            Number { integer: 0xffffffffu64.wrapping_sub(0xffffffff),
                     float: (0xffffffffu64-0xffffffffu64) as f64 }),
        ("0xffffffffffffffff-0",
            Number { integer: 0xffffffffffffffffu64.wrapping_sub(0),
                     float: 0xffffffffffffffffu64 as f64 }),
        ("0xffffffffffffffff-1",
            Number { integer: 0xffffffffffffffffu64.wrapping_sub(1),
                     float: 0xffffffffffffffffu64 as f64 - 1.0 }),
        ("0xffffffffffffffff-0xffffffffffffffff",
            Number { integer: 0xffffffffffffffffu64.wrapping_sub(0xffffffffffffffff),
                     float: 0xffffffffffffffffu64 as f64 - 0xffffffffffffffffu64 as f64 }),

        //
        // Multiply
        //
        ("0*0", Number { integer: 0, float: 0.0 }),
        ("0*1", Number { integer: 0u64.wrapping_mul(1), float: 0.0 }),
        ("1*1", Number { integer: 1, float: 1.0 }),
        ("12*12", Number { integer: 12u64.wrapping_mul(12), float: 144.0 }),
        ("132*100", Number { integer: 13200, float: 13200.0 }),

        ("0xf0f0f0f0*0", Number { integer: 0, float: 0.0 }),
        ("0xf0f0f0f0*1", Number { integer: 0xf0f0f0f0, float: 0xf0f0f0f0u64 as f64 * 1.0 }),
        ("0xf0f0f0f0*0xf0f0f0f0",
            Number { integer: 0xf0f0f0f0u64.wrapping_mul(0xf0f0f0f0),
                     float: 0xf0f0f0f0u64 as f64 * 0xf0f0f0f0u64 as f64 }),
        ("0xf0f0f0f0f0f0f0f0*0", Number { integer: 0, float: 0.0 }),
        ("0xf0f0f0f0f0f0f0f0*1",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_mul(1),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 * 1.0 }),
        ("0xf0f0f0f0f0f0f0f0*0xf0f0f0f0f0f0f0f0",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_mul(0xf0f0f0f0f0f0f0f0),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 * 0xf0f0f0f0f0f0f0f0u64 as f64 }),
        ("0xffffffff*0", Number { integer: 0, float: 0.0 }),
        ("0xffffffff*1", Number { integer: 0xffffffff, float: 0xffffffffu64 as f64 * 1.0 }),
        ("0xffffffff*0xffffffff",
            Number { integer: 0xffffffffu64.wrapping_mul(0xffffffff),
                     float: 0xffffffffu64 as f64 * 0xffffffffu64 as f64 }),
        ("0xffffffffffffffff*0",
            Number { integer: 0, float: 0.0 }),
        ("0xffffffffffffffff*1",
            Number { integer: 0xffffffffffffffffu64.wrapping_mul(1),
                     float: 0xffffffffffffffffu64 as f64 * 1.0 }),
        ("0xffffffffffffffff*0xffffffffffffffff",
            Number { integer: 0xffffffffffffffffu64.wrapping_mul(0xffffffffffffffff),
                     float: 0xffffffffffffffffu64 as f64 * 0xffffffffffffffffu64 as f64 }),
        //
        // Divide
        //
        ("1/1", Number { integer: 1, float: 1.0 }),
        ("12/6", Number { integer: 12u64.wrapping_div(6), float: 2.0 }),
        ("132/100", Number { integer: 132u64.wrapping_div(100), float: 132.0 / 100.0 }),

        ("0xf0f0f0f0/1", Number { integer: 0xf0f0f0f0, float: 0xf0f0f0f0u64 as f64 / 1.0 }),
        ("0xf0f0f0f0/0xf0f0f0f0",
            Number { integer: 0xf0f0f0f0u64.wrapping_div(0xf0f0f0f0),
                     float: 0xf0f0f0f0u64 as f64 / 0xf0f0f0f0u64 as f64 }),
        ("0xf0f0f0f0f0f0f0f0/1",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_div(1),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 / 1.0 }),
        ("0xf0f0f0f0f0f0f0f0/0xf0f0f0f0f0f0f0f0",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_div(0xf0f0f0f0f0f0f0f0),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 / 0xf0f0f0f0f0f0f0f0u64 as f64 }),
        ("0xffffffff/32", Number { integer: 0xffffffffu64 / 32u64, float: 0xffffffffu64 as f64 / 32.0 }),
        ("0xffffffff/0xffffffff",
            Number { integer: 0xffffffffu64.wrapping_div(0xffffffff),
                     float: 0xffffffffu64 as f64 / 0xffffffffu64 as f64 }),
        ("0xffffffffffffffff/1",
            Number { integer: 0xffffffffffffffffu64.wrapping_div(1),
                     float: 0xffffffffffffffffu64 as f64 / 1.0 }),
        ("0xffffffffffffffff/0xffffffffffffffff",
            Number { integer: 0xffffffffffffffffu64.wrapping_div(0xffffffffffffffff),
                     float: 0xffffffffffffffffu64 as f64 / 0xffffffffffffffffu64 as f64 }),
        // Remainder
        ("1%1", Number { integer: 0, float: 0.0 }),
        ("12%6", Number { integer: 12u64.wrapping_rem(6), float: 12.0 % 6.0 }),
        ("132%100", Number { integer: 132u64.wrapping_rem(100), float: 132.0 % 100.0 }),

        ("0xf0f0f0f0%1", Number { integer: 0xf0f0f0f0u64.wrapping_rem(1), float: 0xf0f0f0f0u64 as f64 % 1.0 }),
        ("0xf0f0f0f0%0xf0f0f0f0",
            Number { integer: 0xf0f0f0f0u64.wrapping_rem(0xf0f0f0f0),
                     float: 0xf0f0f0f0u64 as f64 % 0xf0f0f0f0u64 as f64 }),
        ("0xf0f0f0f0f0f0f0f0%3",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_rem(3),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 % 3.0 }),
        ("0xf0f0f0f0f0f0f0f0%0xf1f2f3f4f5f6f7f8",
            Number { integer: 0xf0f0f0f0f0f0f0f0u64.wrapping_rem(0xf1f2f3f4f5f6f7f8),
                     float: 0xf0f0f0f0f0f0f0f0u64 as f64 % 0xf1f2f3f4f5f6f7f8u64 as f64 }),
        ("0xffffffff%255", Number { integer: 0xffffffffu64.wrapping_rem(255), float: 0xffffffffu64 as f64 % 255.0 }),
        ("0xffffffff%0xffffffff",
            Number { integer: 0xffffffffu64.wrapping_rem(0xffffffff),
                     float: 0xffffffffu64 as f64 % 0xffffffffu64 as f64 }),
        ("0xffffffffffffffff%1",
            Number { integer: 0xffffffffffffffffu64.wrapping_rem(1),
                     float: 0xffffffffffffffffu64 as f64 % 1.0 }),
        ("0xffffffffffffffff%0xf0f0f0f0f0f0f0f0",
            Number { integer: 0xffffffffffffffffu64.wrapping_rem(0xf0f0f0f0f0f0f0f0),
                     float: 0xffffffffffffffffu64 as f64 % 0xf0f0f0f0f0f0f0f0u64 as f64 }),
        ("0xffffffffffffffff%0xffffffffffffffff",
            Number { integer: 0xffffffffffffffffu64.wrapping_rem(0xffffffffffffffff),
                     float: 0xffffffffffffffffu64 as f64 % 0xffffffffffffffffu64 as f64 }),

        // Left shift
        // Right shift
        // Less than
        // Less than or equal
        // Greater than
        // Greater than or equal
        // Equal
        // Not equal
        // Bitwise AND
        // Bitwise XOR
        // Bitwise OR
        // Logical AND
        // Logical OR
    ];
    for expr_res in expr_results {
        test_valid_expr(&expr_res.0, &expr_res.1);
    }
}

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
        test_valid_expr(&expr_res.0, &expr_res.1);
    }
}

#[test]
fn valid_exprs_eval_fail() {
    // These are expressions that are syntactically valid but guaranteed to fail during
    // evaluation. E.g "1/0" is perfectly valid syntax but fails due to division by zero.
    // These must never produce errors during the parsing phase.
    use ExprErrorKind::*;
    let expr_results = vec![
        ("0/0", ExprError { idx_expr: 0, kind: FailedEvaluation, message: String::new() }),
        ("1/0", ExprError { idx_expr: 0, kind: FailedEvaluation, message: String::new() }),
        ("2/0", ExprError { idx_expr: 0, kind: FailedEvaluation, message: String::new() }),
        ("0xffffffffffffffff/0", ExprError { idx_expr: 0, kind: FailedEvaluation, message: String::new() }),
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
    // TODO: Try split this into logical categories.
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
        ("!-0", ExprError { idx_expr: 0, kind: MissingOperand, message: String::new() }),
        ("~-0", ExprError { idx_expr: 0, kind: MissingOperand, message: String::new() }),
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

