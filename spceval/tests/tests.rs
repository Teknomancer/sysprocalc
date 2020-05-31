use spceval::{self, Number};

#[test]
fn valid_expressions() {
    let expr_results = vec![
        ("2+2", Number{ integer: 4, float: 4.0 }),
        ("(2+2)", Number{ integer: 4, float: 4.0 }),
        ("1+2*3", Number{ integer: 7, float: 7.0 }),
        ("(1+2)*3", Number{ integer: 9, float: 9.0 }),
        ("(1+2)*(5-1)", Number{ integer: 12, float: 12.0 }),
        ("0xe + 1", Number{ integer: 0xf, float: 15.0 }),
        ("0n100 + 4", Number{ integer: 8, float: 8.0 }),
        ("0o11 + 1", Number{ integer: 10, float: 10.0 }),
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

