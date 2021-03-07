mod functions;
mod operators;
use functions::{FUNCS, Func};
use operators::{OPERS, Oper, OperKind, OperAssoc};

use std::fmt;
use std::collections::VecDeque;
use std::convert::TryFrom;
use log::{trace, debug};   // others: {warn,info}
use arrayvec::ArrayString;

extern crate static_assertions as sa;

// Number of tokens to pre-allocate per ExprCtx.
const PRE_ALLOC_TOKENS: usize = 16;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExprErrorKind {
    EmptyExpr,
    FailedEvaluation,
    InvalidExpr,
    InvalidParamCount,
    InvalidParamType,
    MismatchParenthesis,
    MissingFunction,
    MissingOperand,
    MissingOperator,
    MissingOperatorOrFunction,
    MissingParenthesis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprError {
    idx_expr: usize,
    kind: ExprErrorKind,
    message: String,
}

impl ExprError {
    pub fn index(&self) -> usize {
        self.idx_expr
    }

    pub fn kind(&self) -> ExprErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

pub fn evaluate(str_expr: &str) -> Result<Number, ExprError> {
    let mut expr_ctx = parse_expr(str_expr)?;
    evaluate_expr(&mut expr_ctx)
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Number {
    pub integer: u64,
    pub float: f64,
}

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_errkind = match self.kind {
            ExprErrorKind::EmptyExpr => "expression empty",
            ExprErrorKind::FailedEvaluation => "evaluation failed",
            ExprErrorKind::InvalidExpr => "invalid character",
            ExprErrorKind::InvalidParamCount => "incorrect number of parameters",
            ExprErrorKind::InvalidParamType => "invalid parameter type",
            ExprErrorKind::MismatchParenthesis => "parenthesis mismatch",
            ExprErrorKind::MissingFunction => "function missing",
            ExprErrorKind::MissingOperand => "operand missing",
            ExprErrorKind::MissingOperator => "operator missing",
            ExprErrorKind::MissingOperatorOrFunction => "operator or function missing",
            ExprErrorKind::MissingParenthesis => "parenthesis missing",
        };
        write!(f, "{} {}", str_errkind, self.message)
    }
}

#[derive(Debug, Copy, Clone)]
struct NumToken {
    number: Number,
    idx_expr: usize,
}

#[derive(Copy, Clone)]
struct OperToken {
    idx_oper: usize,
    idx_expr: usize,
}

impl fmt::Debug for OperToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.idx_oper < OPERS.len() {
            write!(f, "'{}'", OPERS[self.idx_oper].name)
        } else {
            write!(f, "Invalid Index {}", self.idx_oper)
        }
    }
}

#[derive(Copy, Clone)]
struct FuncToken {
    idx_func: usize,
    idx_expr: usize,
    params: u8,
}

impl fmt::Debug for FuncToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.idx_func < FUNCS.len() {
            write!(f, "'{}'", &FUNCS[self.idx_func].name)
        } else {
            write!(f, "Invalid Index {}", self.idx_func)
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Num(NumToken),
    Oper(OperToken),
    Func(FuncToken),
}

struct ExprCtx {
    queue_output: VecDeque<Token>,
    stack_op: Vec<Token>,
}

impl TryFrom<Token> for NumToken {
    type Error = &'static str;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Num(num_token) => Ok(num_token),
            _ => Err("not a number token"),
        }
    }
}

impl TryFrom<Token> for OperToken {
    type Error = &'static str;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Oper(oper_token) => Ok(oper_token),
            _ => Err("not an operator token"),
        }
    }
}

impl TryFrom<Token> for FuncToken {
    type Error = &'static str;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Func(func_token) => Ok(func_token),
            _ => Err("not a function token"),
        }
    }
}

impl ExprCtx {
    fn new() -> Self {
        ExprCtx {
            queue_output: VecDeque::with_capacity(PRE_ALLOC_TOKENS),
            stack_op: Vec::with_capacity(PRE_ALLOC_TOKENS)
        }
    }

    fn pop_to_output_queue(&mut self) {
        let token = self.stack_op.pop();
        debug_assert!(token.is_some());
        self.queue_output.push_back(token.unwrap());
    }

    fn pop_all_to_output_queue(&mut self) -> Result<(), ExprError> {
        while let Some(ref_token) = self.stack_op.last() {
            // If the stack has an open parenthesis, we have a parenthesis mismatch.
            match ref_token {
                Token::Oper(OperToken { idx_oper, idx_expr }) => {
                    debug_assert!(*idx_oper < OPERS.len());
                    let oper = &OPERS[*idx_oper];
                    if oper.kind == OperKind::OpenParen {
                        let message = format!("for opening parenthesis at {}", *idx_expr);
                        trace!("Parenthesis mismatch {}", message);
                        return Err(ExprError {
                            idx_expr: *idx_expr,
                            kind: ExprErrorKind::MismatchParenthesis,
                            message
                        });
                    } else {
                        self.pop_to_output_queue();
                    }
                }
                _ => self.pop_to_output_queue(),
            }
        }
        Ok(())
    }

    fn pop_func_from_op_stack(&mut self) -> Option<FuncToken> {
        // If a function preceeds the open parenthesis, pop it to the output queue.
        if let Some(Token::Func(_)) = self.stack_op.last() {
            // We safely unwrap both the token from the stack as well as the result from
            // the try_from() because we've already checked that the token on the top of
            // the stack is a function token.
            Some(FuncToken::try_from(self.stack_op.pop().unwrap()).unwrap())
        } else {
            None
        }
    }

    fn push_func_to_op_stack(&mut self, func_token: FuncToken) -> Result<(), ExprError> {
        let func = &FUNCS[func_token.idx_func];
        if func.params.contains(&func_token.params) {
            self.stack_op.push(Token::Func(func_token));
            Ok(())
        } else {
            // Too many or too few parameters passed to the function, bail.
            let message = format!("for function '{}'. expects [{}..{}) parameters, got {} instead",
                                  func.name, func.params.start, func.params.end, func_token.params);
            Err(ExprError {
                idx_expr: func_token.idx_expr,
                kind: ExprErrorKind::InvalidParamCount,
                message
            })
        }
    }

    fn collect_params(&mut self, params: usize, stack_output: &mut Vec<Number> ) -> Option<Vec<Number>> {
        if params > 0 {
            let stack_len = stack_output.len();
            if stack_len >= params {
                let parameters = stack_output.split_off(stack_len - params);
                Some(parameters)
            } else {
                stack_output.clear();
                None
            }
        } else  {
            None
        }
    }

    fn process_open_paren(&mut self, oper_token: OperToken, opt_prev_token: &Option<Token> ) -> Result<(), ExprError> {
        // Previous token if any cannot be a close parenthesis or a number.
        // E.g "(5)(2)" or "5(2)".
        let is_prev_token_invalid = match opt_prev_token {
            Some(Token::Num(_)) => true,
            Some(Token::Oper(OperToken { idx_oper, .. })) => OPERS[*idx_oper].kind == OperKind::CloseParen,
            _ => false,
        };
        if !is_prev_token_invalid {
            self.stack_op.push(Token::Oper(oper_token));
            Ok(())
        } else {
            let message = format!("for open parenthesis at '{}'", oper_token.idx_expr);
            trace!("{:?} {}", ExprErrorKind::MissingOperatorOrFunction, message);
            Err(ExprError {
                idx_expr: oper_token.idx_expr,
                kind: ExprErrorKind::MissingOperatorOrFunction,
                message
            })
        }
    }

    fn process_close_paren(&mut self, oper_token: OperToken, opt_prev_token: &Option<Token> ) -> Result<(), ExprError> {
        // Find matching open parenthesis.
        let mut is_open_paren_found = false;
        while let Some(ref_token) = self.stack_op.last() {
            match ref_token {
                Token::Oper(OperToken { idx_oper, .. })
                    if OPERS[*idx_oper].kind == OperKind::OpenParen =>
                {
                    is_open_paren_found = true;
                    break;
                }
                // Pop any other tokens to the output queue.
                _ => self.pop_to_output_queue(),
            }
        }

        if is_open_paren_found {
            // Discard open parenthesis from the stack.
            self.stack_op.pop().unwrap();

            // Check if a function preceeds the open parenthesis.
            if let Some(mut func_token) = self.pop_func_from_op_stack() {
                // If we've already counted parameters (due to parameter separators), we will fix up
                // the overlapping parameter count here. E.g "avg(5,6,7)" -- the count will be 4
                // (i.e 2 for each parameter separator) but it should be 3 (N/2+1).
                if func_token.params >= 2 {
                    func_token.params /= 2;
                    func_token.params += 1;
                } else {
                    // If the previous token is a number, the function has 1 parameter.
                    // If the previous token is a unary left associative operator, the function has 1 parameter.
                    // Operator parsing code should've verified the unary operator has a valid parameter.
                    // Any other token implies an invalid sequence and we count it as 0 parameters.
                    func_token.params = match opt_prev_token {
                        Some(Token::Num(_)) => 1,
                        Some(Token::Oper(OperToken { idx_oper, .. }))
                            if OPERS[*idx_oper].assoc == OperAssoc::Left && OPERS[*idx_oper].params == 1 => 1,
                        _ => 0,
                    }
                }
                self.push_func_to_op_stack(func_token)?;
            }
            Ok(())
        } else {
            // If we didn't find a matching opening parenthesis, bail.
            let message = format!("for closing parenthesis at {}", oper_token.idx_expr);
            trace!("Parenthesis mismatch {}", message);
            Err(ExprError {
                idx_expr: oper_token.idx_expr,
                kind: ExprErrorKind::MismatchParenthesis,
                message
            })
        }
    }

    fn process_param_sep(&mut self, oper_token: OperToken) -> Result<(), ExprError> {
        // Find the previous open parenthesis.
        let oper = &OPERS[oper_token.idx_oper];
        while let Some(ref_token) = self.stack_op.last() {
            match ref_token {
                Token::Oper(OperToken { idx_oper, .. })
                    if OPERS[*idx_oper].kind == OperKind::OpenParen => break,
                _ => self.pop_to_output_queue(),
            }
        }

        // If a token exists at the top of the stack, it's an open parenthesis (guaranteed by loop above).
        // We debug asserted below for paranoia.
        if self.stack_op.last().is_some() {
            let paren_token = self.stack_op.pop().unwrap();
            if cfg!(debug_assertions)
            {
                let oper_paren = OperToken::try_from(paren_token).unwrap();
                debug_assert!(OPERS[oper_paren.idx_oper].kind == OperKind::OpenParen);
            }

            // If a function preceeds the open parenthesis, increment its parameter count by 2
            // and re-push the function and the previously popped open parenthesis back to the op stack.
            if let Some(mut func_token) = self.pop_func_from_op_stack() {
                func_token.params += 2;
                self.stack_op.push(Token::Func(func_token));
                self.stack_op.push(paren_token);
                Ok(())
            } else {
                // No function preceeding open parenthesis for a parameter separator, e.g. "(32,5)"
                let message = format!("for parameter separator '{}' at {}", oper.name, oper_token.idx_expr);
                trace!("{:?} {}", ExprErrorKind::MissingFunction, message);
                Err(ExprError {
                    idx_expr: oper_token.idx_expr,
                    kind: ExprErrorKind::MissingFunction,
                    message
                })
            }
        } else {
            // No matching open parenthesis for the parameter separator, e.g. "32,4".
            let message = format!("for parameter separator '{}' at {}", oper.name, oper_token.idx_expr);
            trace!("{:?} {}", ExprErrorKind::MissingParenthesis, message);
            Err(ExprError {
                idx_expr: oper_token.idx_expr,
                kind: ExprErrorKind::MissingParenthesis,
                message
            })
        }
    }

    fn process_regular_oper(&mut self, oper_token: OperToken, opt_prev_token: &Option<Token>) -> Result<(), ExprError> {
        let oper = &OPERS[oper_token.idx_oper];
        // Validate left associative operator.
        // We could squeeze this into parse_oper() but doing it here gives us better
        // error messages in some cases (see integration test).
        if oper.assoc == OperAssoc::Left {
            // Assuming we've parsed left-associative operator "<<".
            // Rules for previous token are:
            // 1. It must exist. E.g. "<< 2" is invalid but we've already handled this in parse_oper().
            //    Debug asserted below for parnoia.
            // 2. Must not be an operator (although close parenthesis is allowed).
            //    E.g. "/ << 2" and "( << 2" are always invalid but ") << 2" may be part of a valid expr.
            // 3. Must not be a right associative operator.
            debug_assert!(opt_prev_token.is_some());
            match opt_prev_token {
                Some(Token::Oper(OperToken { idx_oper, .. }))
                    if OPERS[*idx_oper].kind != OperKind::CloseParen =>
                {
                    let message = format!("for operator '{}' at {}", oper.name, oper_token.idx_expr);
                    trace!("{:?} {}", ExprErrorKind::MissingOperand, message);
                    return Err(ExprError {
                        idx_expr: oper_token.idx_expr,
                        kind: ExprErrorKind::MissingOperand,
                        message
                    });
                }
                _ => (),
            }
        }

        while let Some(ref_token) = self.stack_op.last() {
            match ref_token {
                Token::Oper(OperToken { idx_oper, .. }) => {
                    let token_stack_oper = &OPERS[*idx_oper];
                    debug_assert!(token_stack_oper.kind != OperKind::CloseParen);
                    if token_stack_oper.kind == OperKind::OpenParen {
                        break;
                    } else if token_stack_oper.prec < oper.prec
                               || (oper.assoc == OperAssoc::Left && oper.prec == token_stack_oper.prec) {
                        // Pop operator with higher priority (depending on associativity) to the output queue.
                        self.pop_to_output_queue();
                    } else {
                        break;
                    }
                }

                // Pop functions (which always take priority over a normal operators) to the output queue.
                Token::Func(_) => self.pop_to_output_queue(),

                _ => break,
            }
        }
        self.stack_op.push(Token::Oper(oper_token));
        Ok(())
    }

    #[inline]
    fn process_oper(&mut self, oper_token: OperToken, opt_prev_token: &Option<Token> ) -> Result<(), ExprError> {
        debug_assert!(oper_token.idx_oper < OPERS.len());
        let oper = &OPERS[oper_token.idx_oper];
        match oper.kind {
            OperKind::OpenParen => self.process_open_paren(oper_token, opt_prev_token)?,
            OperKind::CloseParen => self.process_close_paren(oper_token, opt_prev_token)?,
            OperKind::ParamSep => self.process_param_sep(oper_token)?,
            _ => self.process_regular_oper(oper_token, opt_prev_token)?,
        }
        Ok(())
    }
}

fn parse_function(str_expr: &str, funcs: &[Func]) -> Option<usize> {
    debug_assert_eq!(str_expr.trim_start_matches(char::is_whitespace), str_expr);
    // Todo: Sort and use binary search if function table grows.
    let mut is_found = false;
    let mut idx_found = 0;
    for (idx, func) in funcs.iter().enumerate() {
        // If this is the first occurrence of this function, record where we found it.
        // Otherwise, record the currently found function only if its length exceeds that
        // of a previously found one (e.g., find "bits" and not stop at "bit").
        if str_expr.starts_with(func.name)
           && (!is_found || func.name.len() > funcs[idx_found].name.len()) {
            idx_found = idx;
            is_found = true;
        }
    }

    if is_found {
        trace!("found {}", &funcs[idx_found].help);
        Some(idx_found)
    } else {
        None
    }
}

fn parse_num(str_expr: &str) -> (Option<Number>, usize) {
    debug_assert_eq!(str_expr.trim_start_matches(char::is_whitespace), str_expr);

    let mut radix: u32 = 10;
    let mut len_prefix = 0;
    let mut iter_expr = str_expr.chars().peekable();

    // Parse any prefix that is explicitly part of the given expression.
    if str_expr.starts_with('0') {
        len_prefix += 1;
        iter_expr.next();
        if let Some(n) = iter_expr.peek() {
            match n {
                'x' => { len_prefix += 1; iter_expr.next(); radix = 16; }
                'b' => { len_prefix += 1; iter_expr.next(); radix = 2; }
                'o' => { len_prefix += 1; iter_expr.next(); radix = 8; }
                _ => (),
            }
        } else {
            return (Some(Number { integer: 0, float: 0.0 }), 1);
        }
    }

    const MAX_DIGITS: usize = 64 + b"0b".len();
    const STR_SIZE: usize = 72; // Allowed array sizes (for u8) in https://docs.rs/arrayvec/0.3.25/src/arrayvec/array.rs.html

    // Rust string to number conversion functions do not grok prefixes (e.g., "0xf" will have
    // to be fed to it as just "f"). So we aggregate the number into 'str_num' and keep track
    // of the length of any prefix that's already part of the expression in 'len_prefix' (as
    // done above). This also has a side effect in making the loop below faster as we eliminate
    // checks that doesn't need to happen on every iteration.
    let mut str_num = ArrayString::<[_;STR_SIZE]>::new();
    let mut has_dec_pt = false;
    let mut is_fp_exp_notation = false;
    let mut is_fp_exp_sign = false;
    debug_assert!(radix != 0);
    sa::const_assert!(MAX_DIGITS < STR_SIZE); // Code below relies on this otherwise we may panic at runtime.

    // 'consumed' should contain the number of characters consumed by parsing this number
    // including whitespace. We count all the characters ourselves since we need to count
    // whitespaces anyway. Counting all the characters has the added benefit of avoiding
    // a function call to str_num.len() with only a cost of one extra sub on in the break
    // condition in the loop below.
    let mut consumed = len_prefix;
    for chr in iter_expr {
        consumed += 1;
        if consumed > MAX_DIGITS {
            return (None, 0);
        } else if !chr.is_whitespace() {
            if chr.is_digit(radix) {
                str_num.push(chr);
            } else if chr == '.' && radix == 10 && !has_dec_pt {
                has_dec_pt = true;
                str_num.push(chr);
            } else if (chr == 'e' || chr == 'E') && has_dec_pt  {
                // Floating point exponent notation (e.g., "2.5e10" or "2.5E-10").
                str_num.push(chr);
                is_fp_exp_notation = true;
            } else if (chr == '+' || chr == '-') && is_fp_exp_notation && !is_fp_exp_sign {
                // Floating point exponent notation (e.g, +/- power-of character).
                str_num.push(chr);
                is_fp_exp_sign = true;
            } else {
                consumed -= 1;
                break;
            }
        }
    }

    if str_num.is_empty() {
        if len_prefix == 1 {
            // The number is "0" followed by some non-numeric character, return 0.
            (Some(Number { integer: 0, float: 0.0 }), 1)
        } else {
            // No numeric characters with/without prefix, it's invalid (e.g "0x", "0n" or "/").
            (None, 0)
        }
    } else if str_num.ends_with('.') {
        // Number ends in a decimal point, return invalid.
        (None, 0)
    }
    else if !has_dec_pt {
        // Integer.
        match u64::from_str_radix(&str_num, radix) {
            Ok(v) => (Some(Number { integer: v, float: v as f64 }), consumed),
            _ => (None, 0),
        }
    } else {
        // Float.
        // If the float is (+/-)Inf/NaN or otherwise not representable in a u64, casting it
        // results in 0. Right now, I don't know a fool proof way of determining this.
        // TODO: We might also want to consider aborting parsing here in the Inf/NaN case.
        use std::str::FromStr;
        match f64::from_str(&str_num) {
            Ok(v) => (Some(Number { integer: v as u64, float: v }), consumed),
            _ => (None, 0),
        }
    }
}

fn parse_oper(str_expr: &str, opers: &[Oper], opt_prev_token: &Option<Token> ) -> Option<usize> {
    debug_assert_eq!(str_expr.trim_start_matches(char::is_whitespace), str_expr);

    let mut is_found = false;
    let mut idx_found = 0;

    for (idx, op) in opers.iter().enumerate() {
        // If this is the first occurrence of this operator, record where we found it.
        // Otherwise, record the currently found operator only if its length exceeds that
        // of a previously found one (e.g., find "<<" and not stop at "<").
        if str_expr.starts_with(op.name)
           && (!is_found
               || op.name.len() > opers[idx_found].name.len()) {
            // Is this a left associative operator, ensure a previous token exists and that
            // it's not an operator (other than close parenthesis), otherwise skip finding
            // it as a valid operator.
            if op.assoc == OperAssoc::Left {
                match opt_prev_token {
                    // E.g. "<<4" or ",5".
                    None => continue,
                    Some(Token::Oper(OperToken { idx_oper, .. })) => {
                        debug_assert!(*idx_oper < opers.len());
                        // E.g. ")-5" when parsing "-" can be valid. So don't skip finding "-".
                        // E.g. "(<<7" when parsing "<<" is invalid, so skip finding it.
                        if opers[*idx_oper].assoc == OperAssoc::Left
                            || opers[*idx_oper].kind == OperKind::OpenParen
                            || opers[*idx_oper].kind == OperKind::ParamSep {
                            continue;
                        }
                    }
                    _ => (),
                }
            }
            // If this is a right associative operator, ensure if a previous token exists
            // that it's not a right associative unary operator. If it is, it's a malformed
            // expression like "2+++4". Note: "2++4" is 2+(+4), i.e. 2 plus unary plus 4 which is valid.
            //
            // I've got rid of post/pre inc/dec. operators but this does handle the case if I add it back.
            // Maybe error messages might not be great.
            else if op.assoc == OperAssoc::Right {
                if let Some(Token::Oper(OperToken { idx_oper, .. })) = opt_prev_token {
                    if opers[*idx_oper].assoc == OperAssoc::Right {
                        continue;
                    }
                }
            }
            is_found = true;
            idx_found = idx;
        }
    }

    if is_found {
        trace!("found '{}' - {}", opers[idx_found].name, opers[idx_found].help);
        Some(idx_found)
    } else {
        None
    }
}

fn check_prev_token_not_function(opt_prev_token: &Option<Token>) -> Result<(), ExprError> {
    match opt_prev_token {
        Some(Token::Func(FuncToken { idx_func, idx_expr, .. })) => {
            let idx_open_paren = idx_expr + FUNCS[*idx_func].name.len();
            let message = format!("at {} for function '{}'", idx_open_paren, &FUNCS[*idx_func].name);
            trace!("{:?} {}", ExprErrorKind::MissingParenthesis, message);
            Err(ExprError {
                idx_expr: idx_open_paren,
                kind: ExprErrorKind::MissingParenthesis,
                message
            })
        }
        _ => Ok(())
    }
}

fn check_prev_token_not_number(opt_prev_token: &Option<Token>) -> Result<(), ExprError> {
    match opt_prev_token {
        Some(Token::Num(NumToken { number, idx_expr })) => {
            let message = format!("following number {} at {}", number.float, idx_expr);
            trace!("{:?} {}", ExprErrorKind::MissingOperator, message);
            Err(ExprError {
                idx_expr: *idx_expr,
                kind: ExprErrorKind::MissingOperator,
                message
            })
        }
        _ => Ok(())
    }
}

fn check_prev_token_not_close_paren(opt_prev_token: &Option<Token>) -> Result<(), ExprError> {
    match opt_prev_token {
        Some(Token::Oper(OperToken { idx_oper, idx_expr }))
            if OPERS[*idx_oper].kind == OperKind::CloseParen =>
        {
            let idx_oper_or_func = idx_expr + OPERS[*idx_oper].name.len();
            let message = format!("at {}", idx_oper_or_func);
            trace!("{:?} {}", ExprErrorKind::MissingOperatorOrFunction, message);
            Err(ExprError {
                idx_expr: idx_oper_or_func,
                kind: ExprErrorKind::MissingOperatorOrFunction,
                message
            })
        }
        _ => Ok(())
    }
}

fn check_open_paren_for_func(oper_token: &OperToken, opt_prev_token: &Option<Token>) -> Result<(), ExprError> {
    debug_assert!(oper_token.idx_oper < OPERS.len());
    let oper = &OPERS[oper_token.idx_oper];
    match opt_prev_token {
        Some(Token::Func(FuncToken { idx_func, idx_expr, .. } ))
            if oper.kind != OperKind::OpenParen =>
        {
            let idx_open_paren = idx_expr + FUNCS[*idx_func].name.len();
            let message = format!("at {} for function '{}'", idx_open_paren, &FUNCS[*idx_func].name);
            trace!("{:?} {}", ExprErrorKind::MissingParenthesis, message);
            Err(ExprError {
                idx_expr: idx_open_paren,
                kind: ExprErrorKind::MissingParenthesis,
                message
            })
        }
        _ => Ok(())
    }
}

fn parse_expr(str_expr: &str) -> Result<ExprCtx, ExprError> {
    // We iterate by characters here because we want to know the index of every token.
    // The index is primarily for reporting parsing and evaluation errors.
    // If we didn't need to store the index, we can easily loop, trim_start whitespaces,
    // and just re-assign 'str_subexpr' to the string slice given by parse_num().
    let mut expr_ctx = ExprCtx::new();
    let mut opt_prev_token: Option<Token> = None;
    let mut iter_str = str_expr.char_indices();

    while let Some((idx, chr)) = iter_str.next() {
        // Make sure we are not in the middle of a UTF-8 sequence.
        debug_assert!(str_expr.is_char_boundary(idx));
        if chr.is_whitespace() {
            continue;
        }

        let len_token;
        let str_subexpr = &str_expr[idx..];
        if let (Some(number), len_str) = parse_num(str_subexpr) {
            // If the previous token was a function or a close parenthesis, it's invalid.
            // E.g "avg 32.5" or "(2)3" or "(1).5".
            check_prev_token_not_function(&opt_prev_token)?;
            check_prev_token_not_close_paren(&opt_prev_token)?;
            trace!("number  : {} (0x{:x})", number.integer, number.integer);
            let num_token = NumToken { number, idx_expr: idx };
            expr_ctx.queue_output.push_back(Token::Num(num_token));
            len_token = len_str;
            opt_prev_token = Some(Token::Num(num_token));
        } else if let Some(idx_oper) = parse_oper(str_subexpr, &OPERS, &opt_prev_token) {
            debug_assert!(idx_oper < OPERS.len());
            let oper_token = OperToken { idx_oper, idx_expr: idx };
            // If the previous token was a function, this must be an open parenthesis.
            // E.g "avg +"; otherwise this is an invalid expression.
            check_open_paren_for_func(&oper_token, &opt_prev_token)?;
            trace!("operator: {}", &OPERS[idx_oper].name);
            expr_ctx.process_oper(oper_token, &opt_prev_token)?;
            len_token = OPERS[idx_oper].name.len();
            opt_prev_token = Some(Token::Oper(oper_token));
        } else if let Some(idx_func) = parse_function(str_subexpr, &FUNCS) {
            debug_assert!(idx_func < FUNCS.len());
            // If the previous token was a function or a number, we have an invalid expression.
            // E.g "avg avg" or "5 bit(2)"
            check_prev_token_not_function(&opt_prev_token)?;
            check_prev_token_not_number(&opt_prev_token)?;
            trace!("function: {}", &FUNCS[idx_func].name);
            let func_token = FuncToken { idx_func, idx_expr: idx, params: 0 };
            expr_ctx.stack_op.push(Token::Func(func_token));
            len_token = FUNCS[idx_func].name.len();
            opt_prev_token = Some(Token::Func(func_token));
        } else {
            let message = format!("at {}", idx);
            trace!("{:?} {}", ExprErrorKind::InvalidExpr, message);
            return Err(ExprError {
                idx_expr: idx,
                kind: ExprErrorKind::InvalidExpr,
                message
            });
        }

        if len_token >= 2 {
            iter_str.nth(len_token - 2);
        }
    }

    if expr_ctx.stack_op.is_empty() && expr_ctx.queue_output.is_empty() {
        trace!("'{:?}", ExprErrorKind::EmptyExpr);
        Err(ExprError {
            idx_expr: 0,
            kind: ExprErrorKind::EmptyExpr,
            message: "".to_string()
        })
    } else {
        debug!("Op Stack:");
        for (idx,token) in expr_ctx.stack_op.iter().rev().enumerate() {
            debug!("  stack[{}]: {:?}", expr_ctx.stack_op.len() - 1 - idx, token);
        }
        debug!("Output Queue:");
        for (idx,token) in expr_ctx.queue_output.iter().enumerate() {
            debug!("  queue[{}]: {:?}", idx, token);
        }

        // Pop remaining tokens from op stack to the output queue.
        expr_ctx.pop_all_to_output_queue()?;
        Ok(expr_ctx)
    }
}

fn evaluate_expr(expr_ctx: &mut ExprCtx) -> Result<Number, ExprError> {
    // Pop tokens from the output queue to an output stack and process them.
    let mut stack_output: Vec<Number> = Vec::with_capacity(PRE_ALLOC_TOKENS);
    while let Some(token) = expr_ctx.queue_output.pop_front() {
        match token {
            Token::Num(NumToken { number, .. }) => stack_output.push(number),

            Token::Oper(OperToken { idx_oper, idx_expr }) => {
                debug_assert!(idx_oper < OPERS.len());
                let oper = &OPERS[idx_oper];
                if let Some(parameters) = expr_ctx.collect_params(oper.params as usize, &mut stack_output) {
                    debug_assert!(parameters.len() == oper.params as usize);
                    let res_expr = (oper.evalfn)(idx_expr, &parameters)?;
                    stack_output.push(res_expr);
                } else {
                    let message = format!("for operator '{}' at {}", oper.name, idx_expr);
                    trace!("{:?} {}", ExprErrorKind::InvalidParamCount, message);
                    return Err(ExprError {
                        idx_expr,
                        kind: ExprErrorKind::InvalidParamCount,
                        message
                    });
                }
            }

            Token::Func(FuncToken { idx_func, idx_expr, params }) => {
                debug_assert!(idx_func < FUNCS.len());
                let function = &FUNCS[idx_func];
                if let Some(parameters) = expr_ctx.collect_params(params as usize, &mut stack_output) {
                    debug_assert!(parameters.len() == params as usize);
                    let res_expr = (function.evalfn)(function, idx_expr, &parameters)?;
                    stack_output.push(res_expr);
                } else {
                    let message = format!("for function '{}' at {}", function.name, idx_expr);
                    trace!("{:?} {}", ExprErrorKind::InvalidParamCount, message);
                    return Err(ExprError {
                        idx_expr,
                        kind: ExprErrorKind::InvalidParamCount,
                        message
                    });
                }
            }
        }
    }

    if let Some(n) = stack_output.pop() {
        Ok(n)
    } else {
        let message = "evaluation failed".to_string();
        trace!("{}", message);
        Err(ExprError {
            idx_expr: 0,
            kind: ExprErrorKind::InvalidExpr,
            message
        })
    }
}

#[cfg(test)]
mod unit_tests;

