use super::{parse_num, parse_expr, evaluate_expr, ExprErrorKind};
use super::operators::{OPERS, OperKind, OperAssoc};
use super::functions::{FUNCS, MAX_FN_PARAMS};

#[test]
fn parse_invalid_nums() {
    // Number prefixes and improper decimals shouldn't be parsed as valid numbers.
    let mut vec_nums = vec!["",
                        "x" ,
                        "X" ,
                        "o" ,
                        "O" ,
                        "n" ,
                        "N" ,
                        "." ,
                        "0.",
                        "1.",
                        "2.",
                        "3.",
                        "4.",
                        "5.",
                        "0..",
                        "0x",
                        "0xgff",
                        "0b",
                        "0b210110",
                        "0o",
                        "0o888",
                        "..5",
                        "2.5ee4",
                        "2.5e++4",
                        "2.5ee++4",
                        "2.5e--5",
                        "2..5",
    ];
    // Make sure we never parse operators as valid numbers.
    for i in 0..OPERS.len() {
        vec_nums.push(&OPERS[i].name);
    }
    // Make sure we never parse FUNCS as valid numbers.
    for i in 0..FUNCS.len() {
        vec_nums.push(&FUNCS[i].name);
    }
    for num_res in vec_nums {
        let (number, len_str) = parse_num(num_res);
        assert!(number.is_none(), "{}", num_res);
        assert_eq!(len_str, 0);
    }
}

#[test]
fn parse_valid_nums_u64() {
    let pair_int_result = vec![
        // 0-9
        ("0", 0  ), ("1", 1  ), ("2", 2  ), ("3", 3  ), ("4", 4  ), ("5", 5  ),
        ("6", 6  ), ("7", 7  ), ("8", 8  ), ("9", 9  ),
        // 00-09
        ("01", 1  ), ("02", 2  ), ("03", 3  ), ("04", 4  ), ("05", 5  ),
        ("06", 6  ), ("07", 7  ), ("08", 8  ), ("09", 9  ),
        // 10, 010.
        ("10", 10), ("010", 10),
        // 077, 088.
        ("077", 77), ("088", 88),
        // 0x0-0x9, 0xaA-0xfF.
        ("0x0", 0x0), ("0x1", 0x1), ("0x2", 0x2), ("0x3", 0x3), ("0x4", 0x4), ("0x5", 0x5),
        ("0x6", 0x6), ("0x7", 0x7), ("0x8", 0x8), ("0x9", 0x9), ("0xa", 0xa), ("0xA", 0xa),
        ("0xb", 0xb), ("0xB", 0xb), ("0xc", 0xc), ("0xC", 0xc), ("0xd", 0xd), ("0xD", 0xd),
        ("0xe", 0xe), ("0xE", 0xe), ("0xf", 0xf), ("0xF", 0xf),
        ("0x123"             , 0x123             ),
        ("0x1234"            , 0x1234            ),
        ("0x12345"           , 0x12345           ),
        ("0x123456"          , 0x123456          ),
        ("0x1234567"         , 0x1234567         ),
        ("0x12345678"        , 0x12345678        ),
        ("0x123456789"       , 0x123456789       ),
        ("0x123456789abcdef",  0x123456789abcdef ),
        ("0x0123456789abcdef", 0x0123456789abcdef),
        ("0xabcdef0123456789", 0xabcdef0123456789),
        ("0xffffffff"        , 0xffffffff        ),
        ("0xffffffff0"       , 0xffffffff0       ),
        ("0xffffffff00"      , 0xffffffff00      ),
        ("0xffffffff000"     , 0xffffffff000     ),
        ("0xffffffff0000"    , 0xffffffff0000    ),
        ("0xffffffff00000"   , 0xffffffff00000   ),
        ("0xffffffff000000"  , 0xffffffff000000  ),
        ("0xffffffff0000000" , 0xffffffff0000000 ),
        ("0xffffffff00000000", 0xffffffff00000000),
        ("0xffffffffffffffff", 0xffffffffffffffff),
        ("0xFFFFFFFFFFFFFFFF", 0xffffffffffffffff),
        ("0x00000000ffffffff", 0x00000000ffffffff),
        ("0x00000000fffffff" , 0x00000000fffffff ),
        ("0x00000000ffffff"  , 0x00000000ffffff  ),
        ("0x00000000fffff"   , 0x00000000fffff   ),
        ("0x00000000ffff"    , 0x00000000ffff    ),
        ("0x00000000fff"     , 0x00000000fff     ),
        ("0x00000000ff"      , 0x00000000ff      ),
        ("0x00000000f"       , 0x00000000f       ),
        ("0xffffffff"        , 0xffffffff        ),
        ("0x0fffffff"        , 0x0fffffff        ),
        ("0x1fffffff"        , 0x1fffffff        ),
        ("0x7fffffff"        , 0x7fffffff        ),
        ("0xfffffff0"        , 0xfffffff0        ),
        ("0xfffffff1"        , 0xfffffff1        ),
        ("0xfffffff7"        , 0xfffffff7        ),
        ("0xffffffffffffffff", 0xffffffffffffffff),
        ("0x0fffffffffffffff", 0x0fffffffffffffff),
        ("0x1fffffffffffffff", 0x1fffffffffffffff),
        ("0x7fffffffffffffff", 0x7fffffffffffffff),
        ("0xfffffffffffffff0", 0xfffffffffffffff0),
        ("0xfffffffffffffff1", 0xfffffffffffffff1),
        ("0xfffffffffffffff7", 0xfffffffffffffff7),
        ("0xabcdefabcdefabcd", 0xabcdefabcdefabcd),
        ("0xFEDCBAFEDCBAFEDC", 0xfedcbafedcbafedc),
        // Binary prefix
        ("0b0",  0  ), ("0b1",  1  ), ("0b10", 2  ), ("0b11", 3  ), ("0b100", 4 ),
        ("0b11111111111111111111111111111111", 0xffffffff),
        ("0b1111111111111111111111111111111111111111111111111111111111111111", 0xffffffffffffffff),
        ("0b0000000000000000000000000000000011111111111111111111111111111111", 0xffffffff),
        ("0b1111111111111111111111111111111100000000000000000000000000000000", 0xffffffff00000000),
        ("0b1010101010101010101010101010101010101010101010101010101010101010", 0xaaaaaaaaaaaaaaaa),
        // Octal prefix.
        ("0o0",  0  ), ("0o1",  1  ), ("0o2",  2  ), ("0o3",  3  ), ("0o4",  4  ),
        ("0o5",  5  ), ("0o6",  6  ), ("0o7",  7  ), ("0o7",  7  ),
        ("0o10", 8  ), ("0o11", 9  ),
        ("0o77", 63 ), ("0o100", 64),
        // With whitespaces
        ("5 4 3 2 1", 54321),
        ("0xffff ffff ffff fff7", 0xfffffffffffffff7),
        ("0x 123456789", 0x123456789),
        ("0b 101 000 100", 324),
        ("0b 1 0 0", 4),
        ("0o 1 7 7 1", 1017),
    ];
    for int_res in pair_int_result {
        let (number, len_str) = parse_num(int_res.0);
        assert!(number.is_some(), "failed for ('{}', {})", int_res.0, int_res.1);
        assert_eq!(number.unwrap().integer, int_res.1);
        assert_eq!(len_str, int_res.0.len());
    }
}

#[test]
fn parse_valid_nums_f64() {
    let pair_float_result = vec![
        ("0.0"      , 0.0f64   ),
        ("0.1"      , 0.1f64   ),
        ("0.2"      , 0.2f64   ),
        ("0.3"      , 0.3f64   ),
        ("0.4"      , 0.4f64   ),
        ("0.5"      , 0.5f64   ),
        ("0.6"      , 0.6f64   ),
        ("0.7"      , 0.7f64   ),
        ("0.8"      , 0.8f64   ),
        ("0.9"      , 0.9f64   ),
        ("1.0"      , 1.0f64   ),
        ("1.1"      , 1.1f64   ),
        ("1.2"      , 1.2f64   ),
        ("1.3"      , 1.3f64   ),
        ("1.4"      , 1.4f64   ),
        ("1.5"      , 1.5f64   ),
        ("1.6"      , 1.6f64   ),
        ("1.7"      , 1.7f64   ),
        ("1.8"      , 1.8f64   ),
        ("1.9"      , 1.9f64   ),
        ("10.0"     , 10.0f64  ),
        ("10.1"     , 10.1f64  ),
        ("16.0"     , 16.0f64  ),
        ("015.0"    , 15.0f64  ),
        ("2.5e2"    , 250.0f64 ),
        (".5e+2"    , 50.0f64  ),
        ("1234.5e-2", 12.345f64),
    ];
    for float_res in pair_float_result {
        let (number, len_str) = parse_num(float_res.0);
        assert!(number.is_some(), "failed for ('{}', {})", float_res.0, float_res.1);
        assert_eq!(number.unwrap().float, float_res.1);
        assert_eq!(len_str, float_res.0.len());
    }
}

#[test]
fn is_oper_table_valid() {
    let mut open_paren_count = 0;
    let mut close_paren_count = 0;
    let mut param_sep_count = 0;
    for (idx, oper) in OPERS.iter().enumerate() {
        assert!(oper.params < 3, "Oper '{}' at {} has {} parameters. \
                Opers can have at most 2 parameters.", oper.name, idx, oper.params);
        assert!(oper.kind != OperKind::Regular || oper.params > 0,
                "Regular operator '{}' at {} cannot have 0 parameters.", oper.name, idx);
        assert!(oper.assoc != OperAssoc::Right || oper.params == 1,
                "operator '{}' at {} must have only 1 parameter.", oper.name, idx);

        assert_eq!(oper.name.chars().all(|x| x.is_digit(10)), false,
                   "Oper '{}' invalid. Name cannot contain digits.", oper.name);
        assert_eq!(oper.name.chars().all(|x| x == '_'), false,
                   "Oper '{}' invalid. Name cannot contain '_' character.", oper.name);
        assert_eq!(oper.name.chars().all(|x| x == 'x'), false,
                   "Oper '{}' invalid. Name cannot contain 'x' hexadecimal prefix character.", oper.name);
        assert_eq!(oper.name.chars().all(|x| x == 'n'), false,
                   "Oper '{}' invalid. Name cannot contain 'n' binary prefix character.", oper.name);
        assert_eq!(oper.name.chars().all(|x| x == 'o'), false,
                   "Oper '{}' invalid. Name cannot contain 'o' octal prefix character.", oper.name);

        // Ensure open and close parenthesis operators have Nil associativity.
        match oper.kind {
            OperKind::OpenParen => {
                assert_eq!(oper.assoc, OperAssoc::Nil,
                        "Open parenthesis operator '{}' at {} must have no associativity.", oper.name, idx);
                open_paren_count += 1;
            }
            OperKind::CloseParen => {
                assert_eq!(oper.assoc, OperAssoc::Nil,
                        "Close parenthesis operator '{}' at {} must have no associativity.", oper.name, idx);
                close_paren_count += 1;
            }
            OperKind::ParamSep => param_sep_count += 1,
            _ => (),
        }

        for (idxcmp, opercmp) in OPERS.iter().enumerate() {
            if idxcmp != idx {
                // Ensure no duplicate operators.
                // They can have the same name but must differ in associativity.
                assert!(oper.assoc != opercmp.assoc || oper.name != opercmp.name,
                        "Duplicate operator '{}' at {} and {}", oper.name, idx, idxcmp);

                // Ensure that operators with the same name (but differ in associativity),
                // are ordered such that the one with more parameters is first.
                // E.g., binary "-" is ordered before unary "-".
                // However if they have the same number of parameters (e.g, post and
                // pre-increment "++", then we don't care about order.
                if oper.name == opercmp.name && oper.params != opercmp.params {
                    if oper.params > opercmp.params {
                        assert!(idx < idxcmp,
                                "Invalid ordering of '{}' at {} and {}.\
                                The one with more parameters must be sorted higher.",
                                oper.name, idx, idxcmp)
                    } else {
                        assert!(idx > idxcmp,
                                "Invalid ordering of '{}' at {} and {}.\
                                The one with more parameters must be sorted higher.",
                                oper.name, idx, idxcmp)
                    }
                }
            }
        }
    }

    // Ensure there's exactly one of the following operators in the table.
    assert_eq!(open_paren_count, 1);
    assert_eq!(close_paren_count, 1);
    assert_eq!(param_sep_count, 1);
}

#[test]
fn is_func_table_valid() {
    for (idx, func) in FUNCS.iter().enumerate() {
        assert!(!func.params.contains(&MAX_FN_PARAMS),
                "Function '{}' at {} exceeds maximum parameters of {}. Use/alter the maximum.",
                func.name, idx, MAX_FN_PARAMS);
        assert!(!func.params.is_empty(),
                "Function '{}' at {} must have at least 1 parameter. Use/alter the maximum.",
                func.name, idx);

        assert_eq!(func.name.is_empty(), false,
                "Function at {} invalid. Name cannot be 0 length.", idx);
        assert_eq!(func.name.chars().nth(0).unwrap().is_digit(10), false,
                   "Function '{}' invalid. Name cannot start with digits.", func.name);
        assert_ne!(func.name.chars().nth(0).unwrap(), '_',
                   "Function '{}' invalid. Name cannot start with an '_' character.", func.name);

        // Ensure no duplicate FUNCS names.
        for (idxcmp, funccmp) in FUNCS.iter().enumerate() {
            if idxcmp != idx {
                assert!(func.name != funccmp.name,
                        "Duplicate function '{}' at {} and {}", func.name, idx, idxcmp);
            }
        }
    }
}

#[inline(always)]
fn test_valid_expr_but_eval_fail(str_expr: &str, expr_error_kind: ExprErrorKind) {
    // Parsing should succeed but evaluation must fail and match the specified error.
    let res_parse = parse_expr(str_expr);
    assert!(res_parse.is_ok(), "{}", str_expr);
    let mut expr_ctx = res_parse.unwrap();
    let res_eval = evaluate_expr(&mut expr_ctx);
    assert!(res_eval.is_err(), "{}", str_expr);
    assert_eq!(expr_error_kind, res_eval.err().unwrap().kind, "{}", str_expr);
}

#[test]
fn valid_exprs_eval_fail() {
    // These are expressions that are syntactically valid but guaranteed to fail during
    // evaluation. E.g "1/0" is perfectly valid syntax but fails due to division by zero.
    // These must never produce errors during the parsing phase.
    let expr_results = vec![
        ("0/0", ExprErrorKind::FailedEvaluation),
        ("1/0", ExprErrorKind::FailedEvaluation),
        ("2/0", ExprErrorKind::FailedEvaluation),
        ("0xffffffffffffffff/0", ExprErrorKind::FailedEvaluation),

        //
        // Functions
        //
        // bit
        ("bit(-1)", ExprErrorKind::FailedEvaluation),
        ("bit(64)", ExprErrorKind::FailedEvaluation),
        ("bit(~0)", ExprErrorKind::FailedEvaluation),
        ("bit(0xffffffffffffffff)", ExprErrorKind::FailedEvaluation),
        ("bit(0x7fffffffffffffff)", ExprErrorKind::FailedEvaluation),

        // bits
        ("bits(0,-1)", ExprErrorKind::FailedEvaluation),
        ("bits(-1,-1)", ExprErrorKind::FailedEvaluation),
        ("bits(64,0)", ExprErrorKind::FailedEvaluation),
        ("bits(0,64)", ExprErrorKind::FailedEvaluation),
        ("bits(~0,0)", ExprErrorKind::FailedEvaluation),
    ];
    for expr_res in expr_results {
        test_valid_expr_but_eval_fail(&expr_res.0, expr_res.1);
    }
}

