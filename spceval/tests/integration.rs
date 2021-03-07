use spceval::{Number, ExprErrorKind};

#[inline(always)]
fn test_valid_expr(str_expr: &str, num_expected: &Number) {
    let res_eval = spceval::evaluate(str_expr);
    assert!(res_eval.is_ok(), "{}", str_expr);
    let num_computed = res_eval.unwrap();
    assert_eq!(num_expected.integer, num_computed.integer, "{}", str_expr);
    assert_eq!(num_expected.float, num_computed.float, "{}", str_expr);
}

#[inline(always)]
fn test_invalid_expr(str_expr: &str, expr_error_kind: ExprErrorKind) {
    let res_eval = spceval::evaluate(str_expr);
    assert!(res_eval.is_err(), "{}", str_expr);
    assert_eq!(expr_error_kind, res_eval.err().unwrap().kind(), "{}", str_expr);
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
        // Add
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

        // Subtract
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

        // Multiply
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

        // Divide
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
fn valid_exprs_funcs() {
    let expr_results = vec![
        // avg
        ("avg(0,0)", Number { integer: 0, float: 0 as f64 }),
        ("avg(1,3)", Number { integer: 2, float: 2 as f64 }),
        ("avg(0xf,0x1e,0x2d)", Number { integer: 0x1e, float: 0x1e as f64 }),
        ("avg(1,2,3,4,5,6,7,8,9,10)", Number { integer: 5, float: 5.5f64 }),
        ("avg(35,75,125,25,45) + avg(1,2,3)", Number { integer: 63, float: 63.0 }),

        // bit
        ("bit(0)", Number { integer: 1_u64.wrapping_shl(0), float: 1_u64.wrapping_shl(0) as f64 }),
        ("bit(1)", Number { integer: 1_u64.wrapping_shl(1), float: 1_u64.wrapping_shl(1) as f64 }),
        ("bit(31)", Number { integer: 1_u64.wrapping_shl(31), float: 1_u64.wrapping_shl(31) as f64 }),
        ("bit(63)", Number { integer: 1_u64.wrapping_shl(63), float: 1_u64.wrapping_shl(63) as f64 }),
        ("bit(0) | bit(1) | bit(2)", Number { integer: 7, float: 7.0 }),

        // bits
        ("bits(0,0)", Number { integer: 1, float: 1.0 }),
        ("bits(0,1)", Number { integer: 3, float: 3.0 }),
        ("bits(1,0)", Number { integer: 3, float: 3.0 }),
        ("bits(1,1)", Number { integer: 2, float: 2.0 }),
        ("bits(0,2)", Number { integer: 7, float: 7.0 }),
        ("bits(2,0)", Number { integer: 7, float: 7.0 }),
        ("bits(0,63)", Number { integer: 0xffffffffffffffffu64, float: 0xffffffffffffffffu64 as f64 }),
        ("bits(63,0)", Number { integer: 0xffffffffffffffffu64, float: 0xffffffffffffffffu64 as f64 }),
        ("bits(0,31)", Number { integer: 0xffffffffu64, float: 0xffffffffu64 as f64 }),
        ("bits(31,0)", Number { integer: 0xffffffffu64, float: 0xffffffffu64 as f64 }),
        ("bits(32,63)", Number { integer: 0xffffffff00000000u64, float: 0xffffffff00000000u64 as f64 }),
        ("bits(63,32)", Number { integer: 0xffffffff00000000u64, float: 0xffffffff00000000u64 as f64 }),

        // if
        // TODO

        // sum
        ("sum(0,0)", Number { integer: 0, float: 0 as f64 }),
        ("sum(1,3)", Number { integer: 4, float: 4 as f64 }),
        ("sum(0xffff,0xffffffff)", Number { integer: 0xffff + 0xffffffff_u64, float: (0xffff + 0xffffffff_u64) as f64 }),
        ("sum(-1,4)", Number { integer: 3, float: 3 as f64 }),
        ("sum(-5,-5,10)", Number { integer: 0, float: 0 as f64 }),
        ("sum(10,5) * sum(1,2)", Number { integer: 45, float: 45.0 }),
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
        ("((0x128)) + 0b111", Number { integer: 303, float: 303.0 }),
        ("1*4+(0b1+0xf)", Number { integer: 20, float: 20.0 }),
        (".5*0", Number { integer: 0, float: 0.0 }),
        ("5/(5/(5/(5)))", Number { integer: 1, float: 1.0 }),
        ("212 + (1 * (3 - (4 * 5)))", Number { integer: 195, float: 195.0 }),
        ("0*5", Number { integer: 0, float: 0.0 }),
        ("0/5", Number { integer: 0, float: 0.0 }),
        ("0x f f f f + 0xf ff f", Number { integer: 0x1fffe, float: 0x1fffeu64 as f64 }),
    ];
    for expr_res in expr_results {
        test_valid_expr(&expr_res.0, &expr_res.1);
    }
}

#[test]
fn invalid_exprs() {
    // These are expressions that MUST produce errors in either the parse or evaluation
    // phase. As long as they produce the required errors it's fine. Some expressions
    // like "2 +" fail during evaluation due to the way we parse operators but others
    // like ",5" will fail during parsing.
    // TODO: Try split this into logical categories.
    let expr_results = vec![
        ("", ExprErrorKind::EmptyExpr),
        ("()", ExprErrorKind::EmptyExpr),
        ("2 +", ExprErrorKind::InvalidParamCount),
        ("- -2", ExprErrorKind::MissingOperand),
        ("+ +2", ExprErrorKind::MissingOperand),
        (",2", ExprErrorKind::InvalidExpr),
        ("(", ExprErrorKind::MismatchParenthesis),
        (")", ExprErrorKind::MismatchParenthesis),
        (",", ExprErrorKind::InvalidExpr),
        ("(,", ExprErrorKind::InvalidExpr),
        (",)", ExprErrorKind::InvalidExpr),
        ("(5,", ExprErrorKind::MissingFunction),
        ("(2 +", ExprErrorKind::MismatchParenthesis),
        ("2 + 5)", ExprErrorKind::MismatchParenthesis),
        ("++", ExprErrorKind::MissingOperand),
        ("--", ExprErrorKind::MissingOperand),
        ("0,", ExprErrorKind::MissingParenthesis),
        ("(),", ExprErrorKind::MissingParenthesis),
        ("(0),", ExprErrorKind::MissingParenthesis),
        ("5+()", ExprErrorKind::InvalidParamCount),
        ("-()", ExprErrorKind::InvalidParamCount),
        ("+()", ExprErrorKind::InvalidParamCount),
        (",(),", ExprErrorKind::InvalidExpr),
        ("(),()", ExprErrorKind::MissingParenthesis),
        ("(1),()", ExprErrorKind::MissingParenthesis),
        ("(1),(2)", ExprErrorKind::MissingParenthesis),
        ("(1),(2),", ExprErrorKind::MissingParenthesis),
        ("(1)(2),", ExprErrorKind::MissingOperatorOrFunction),
        ("5(2),", ExprErrorKind::MissingOperatorOrFunction),
        ("5,(2),", ExprErrorKind::MissingParenthesis),
        ("(<<2", ExprErrorKind::InvalidExpr),
        ("(1)2", ExprErrorKind::MissingOperatorOrFunction),
        ("(1).5", ExprErrorKind::MissingOperatorOrFunction),
        ("2 << << 4", ExprErrorKind::InvalidExpr),
        ("2<<<<4", ExprErrorKind::InvalidExpr),
        ("2+ + +4", ExprErrorKind::MissingOperand),
        ("2+++4", ExprErrorKind::MissingOperand),
        (".+2", ExprErrorKind::InvalidExpr),
        (".-2", ExprErrorKind::InvalidExpr),
        ("(.5", ExprErrorKind::MismatchParenthesis),
        (").f", ExprErrorKind::MismatchParenthesis),
        ("(-1).5", ExprErrorKind::MissingOperatorOrFunction),
        ("!-0", ExprErrorKind::MissingOperand),
        ("~-0", ExprErrorKind::MissingOperand),
        ("0 x123", ExprErrorKind::InvalidExpr),
        ("0 n123", ExprErrorKind::InvalidExpr),
        ("0 o1011", ExprErrorKind::InvalidExpr),

        // General function syntax
        (")avg(5,3,5)", ExprErrorKind::MismatchParenthesis),
        ("sum avg(5,3,5)", ExprErrorKind::MissingParenthesis),
        ("bit(5", ExprErrorKind::MismatchParenthesis),
        ("* bit(5)", ExprErrorKind::InvalidExpr),
        ("() + bit(5)", ExprErrorKind::InvalidParamCount),
        ("(sum(2,5)", ExprErrorKind::MismatchParenthesis),
        ("6 sum(2,5)", ExprErrorKind::MissingOperator),
        ("bit(2) 32", ExprErrorKind::MissingOperatorOrFunction),
        ("avg +", ExprErrorKind::MissingParenthesis),
        ("* avg", ExprErrorKind::InvalidExpr),
        ("- avg", ExprErrorKind::InvalidParamCount),
        ("avg *", ExprErrorKind::MissingParenthesis),
        ("avg + sum", ExprErrorKind::MissingParenthesis),
        ("avg() + sum(2,5)", ExprErrorKind::InvalidParamCount),
        ("avg(2,5) + sum()", ExprErrorKind::InvalidParamCount),

        // Functions
        ("avg(123)", ExprErrorKind::InvalidParamCount),
        ("bit(1,2)", ExprErrorKind::InvalidParamCount),
        ("bits(0)", ExprErrorKind::InvalidParamCount),
        ("bits(63)", ExprErrorKind::InvalidParamCount),
        ("bits(64)", ExprErrorKind::InvalidParamCount),
        // TODO if
        ("sum(0xff)", ExprErrorKind::InvalidParamCount),
    ];
    for expr_res in expr_results {
        test_invalid_expr(&expr_res.0, expr_res.1);
    }
}

