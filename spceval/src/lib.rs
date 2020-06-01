use std::fmt;
use std::ops::Range;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::convert::TryFrom;
use log::{trace, debug};   // others: {warn,info}

// Number of tokens to pre-allocate per ExprCtx.
const PRE_ALLOC_TOKENS: usize = 16;

static OPERATORS: [Operator<'static>; 26] = [
    // Precedence 1 (highest priority)
    Operator { kind: OperatorKind::OpenParen,  prec: 1,  params: 0, assoc: OperatorAssoc::Nil,   name: "(",  syntax: "(<expr>)",           help: "Begin expression."             , func: oper_nop },
    Operator { kind: OperatorKind::CloseParen, prec: 1,  params: 0, assoc: OperatorAssoc::Nil,   name: ")",  syntax: "(<expr>)",           help: "End expression."               , func: oper_nop },
    // Precendence 4 (appears in array before 2 because of parsing logic with unary operators)
    Operator { kind: OperatorKind::Regular,    prec: 4,  params: 2, assoc: OperatorAssoc::Left,  name: "+",  syntax: "<expr> + <expr>",    help: "Addition."                     , func: oper_add },
    Operator { kind: OperatorKind::Regular,    prec: 4,  params: 2, assoc: OperatorAssoc::Left,  name: "-",  syntax: "<expr> - <expr>",    help: "Subtraction."                  , func: oper_sub },
    // Precedence 2
    Operator { kind: OperatorKind::Regular,    prec: 2,  params: 1, assoc: OperatorAssoc::Right, name: "+",  syntax: "+<expr>",            help: "Unary plus."                   , func: oper_nop },
    Operator { kind: OperatorKind::Regular,    prec: 2,  params: 1, assoc: OperatorAssoc::Right, name: "-",  syntax: "-<expr>",            help: "Unary minus."                  , func: oper_unary_minus },
    Operator { kind: OperatorKind::Regular,    prec: 2,  params: 1, assoc: OperatorAssoc::Right, name: "!",  syntax: "!<expr>",            help: "Logical NOT."                  , func: oper_logical_not },
    Operator { kind: OperatorKind::Regular,    prec: 2,  params: 1, assoc: OperatorAssoc::Right, name: "~",  syntax: "~<expr>",            help: "Bitwise NOT."                  , func: oper_bit_not },
    // Precedence 3
    Operator { kind: OperatorKind::Regular,    prec: 3,  params: 2, assoc: OperatorAssoc::Left,  name: "*",  syntax: "<expr> * <expr>",    help: "Multiplication."               , func: oper_mul },
    Operator { kind: OperatorKind::Regular,    prec: 3,  params: 2, assoc: OperatorAssoc::Left,  name: "/",  syntax: "<expr> / <expr>",    help: "Division."                     , func: oper_div },
    Operator { kind: OperatorKind::Regular,    prec: 3,  params: 2, assoc: OperatorAssoc::Left,  name: "%",  syntax: "<expr> % <expr>",    help: "Remainder."                    , func: oper_rem },
    // Precedence 5
    Operator { kind: OperatorKind::Regular,    prec: 5,  params: 2, assoc: OperatorAssoc::Left,  name: "<<", syntax: "<expr> << <expr>",   help: "Bitwise left-shift."           , func: oper_bit_lshift },
    Operator { kind: OperatorKind::Regular,    prec: 5,  params: 2, assoc: OperatorAssoc::Left,  name: ">>", syntax: "<expr> >> <expr>",   help: "Bitwise right-shift."          , func: oper_bit_rshift },
    // Precedence 6
    Operator { kind: OperatorKind::Regular,    prec: 6,  params: 2, assoc: OperatorAssoc::Left,  name: "<",  syntax: "<expr> < <expr>",    help: "Less-than."                    , func: oper_lt },
    Operator { kind: OperatorKind::Regular,    prec: 6,  params: 2, assoc: OperatorAssoc::Left,  name: "<=", syntax: "<expr> <= <expr>",   help: "Less-than-or-equals."          , func: oper_lte },
    Operator { kind: OperatorKind::Regular,    prec: 6,  params: 2, assoc: OperatorAssoc::Left,  name: ">",  syntax: "<expr> > <expr>",    help: "Greater-than."                 , func: oper_gt },
    Operator { kind: OperatorKind::Regular,    prec: 6,  params: 2, assoc: OperatorAssoc::Left,  name: ">=", syntax: "<expr> >= <expr>",   help: "Greater-than-or-equals."       , func: oper_gte },
    // Precedence 7
    Operator { kind: OperatorKind::Regular,    prec: 7,  params: 2, assoc: OperatorAssoc::Left,  name: "==", syntax: "<expr> == <expr>",   help: "Equals."                       , func: oper_eq },
    Operator { kind: OperatorKind::Regular,    prec: 7,  params: 2, assoc: OperatorAssoc::Left,  name: "!=", syntax: "<expr> != <expr>",   help: "Not-equals."                   , func: oper_ne },
    // Precedence 8
    Operator { kind: OperatorKind::Regular,    prec: 8,  params: 2, assoc: OperatorAssoc::Left,  name: "&",  syntax: "<expr> & <expr>",    help: "Bitwise AND."                  , func: oper_bit_and },
    // Precedence 9
    Operator { kind: OperatorKind::Regular,    prec: 9,  params: 2, assoc: OperatorAssoc::Left,  name: "^",  syntax: "<expr> ^ <expr>",    help: "Bitwise XOR."                  , func: oper_bit_xor },
    // Precedence 10
    Operator { kind: OperatorKind::Regular,    prec: 10, params: 2, assoc: OperatorAssoc::Left,  name: "|",  syntax: "<expr> | <expr>",    help: "Bitwise OR."                   , func: oper_bit_or },
    // Precedence 11
    Operator { kind: OperatorKind::Regular,    prec: 11, params: 2, assoc: OperatorAssoc::Left,  name: "&&", syntax: "<expr> && <expr>",   help: "Logical AND."                  , func: oper_nop },
    // Precedence 12
    Operator { kind: OperatorKind::Regular,    prec: 12, params: 2, assoc: OperatorAssoc::Left,  name: "||", syntax: "<expr> || <expr>",   help: "Logical OR."                   , func: oper_nop },
    // Precedence 13
    Operator { kind: OperatorKind::VarAssign,  prec: 13, params: 2, assoc: OperatorAssoc::Left,  name: "=",  syntax: "<var> = <expr>",     help: "Variable assignment."          , func: oper_nop },
    // Precedence 14
    Operator { kind: OperatorKind::ParamSep,   prec: 14, params: 2, assoc: OperatorAssoc::Left,  name: ",",  syntax: "<param1>, <param2>", help: "Function parameter separator." , func: oper_nop },
];

const MAX_FN_PARAMS: u8 = u8::max_value();
static FUNCTIONS: [Function<'static>; 3] = [
    Function {
        name: "sum",
        params: Range { start: 2, end: MAX_FN_PARAMS },
        syntax: "<n1>,<n2>[,<n3>...<nX>]",
        help: "Sum",
        func: func_sum,
    },
    Function {
        name: "avg",
        params: Range { start: 2, end: MAX_FN_PARAMS },
        syntax: "<n1>,<n2>[,<n3>...<nX>]",
        help: "Average",
        func: func_avg,
    },
    Function {
        name: "if",
        params: Range { start: 3, end: 4 },
        syntax: "<cond>,<n1>,<n2>",
        help: "If <cond> is true, returns <n1> else <n2>",
        func: func_dummy,
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExprErrorKind {
    EmptyExpr,
    InvalidExpr,
    InvalidParamCount,
    InvalidParamType,
    MismatchParenthesis,
    MissingFunction,
    MissingOperatorOrFunction,
    MissingParenthesis,
    FatalInternal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprError {
    pub idx_expr: usize,
    pub kind: ExprErrorKind,
    pub message: String,
}

impl ExprError {
    pub fn kind(&self) -> &ExprErrorKind {
        &self.kind
    }
}

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_errkind = match self.kind {
            ExprErrorKind::EmptyExpr => "expression empty",
            ExprErrorKind::InvalidExpr => "invalid character",
            ExprErrorKind::InvalidParamCount => "incorrect number of parameters",
            ExprErrorKind::InvalidParamType => "invalid parameter type",
            ExprErrorKind::MismatchParenthesis => "parenthesis mismatch",
            ExprErrorKind::MissingFunction => "function missing",
            ExprErrorKind::MissingOperatorOrFunction => "operator or function missing",
            ExprErrorKind::MissingParenthesis => "parenthesis missing",
            ExprErrorKind::FatalInternal => "fatal internal error",
        };
        write!(f, "{} {}", str_errkind, self.message)
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Number {
    pub integer: u64,
    pub float: f64,
}

type PfnOper = fn(&[Number]) -> Result<Number, ExprError>;
type PfnFunc = fn(&[Number]) -> Result<Number, ExprError>;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum OperatorAssoc {
    Nil,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum OperatorKind {
    Regular,
    OpenParen,
    CloseParen,
    ParamSep,
    VarAssign,
}

struct Operator<'a> {
    kind: OperatorKind,
    prec: u8,
    params: u8,
    assoc: OperatorAssoc,
    name: &'a str,
    syntax: &'a str,
    help: &'a str,
    func: PfnOper,
}

struct Function<'a> {
    name: &'a str,
    params: Range<u8>,
    syntax: &'a str,
    help: &'a str,
    func: PfnFunc,
}

// Eq specifies that the equality relationship defined by PartialEq is a total equality.
impl<'a> Eq for Operator<'a> {}

// PartialEq is required by PartialOrd which is required for sorting.
impl<'a> PartialEq for Operator<'a> {
    fn eq(&self, other: &Operator) -> bool {
           self.kind == other.kind
        && self.prec == other.prec
        && self.params == other.params
        && self.assoc == other.assoc
        && self.name == other.name
    }
}

// Order (sort) by operator name in reverse so that longer operator names are be sorted before
// shorter ones. This is so that while iterating operators, we might want to encounter longer
// operator names before shorter ones (e.g., "<=" before "<" and "<<" before "<") regardless of
// their precedence. This is so if we use 'starts_with' and find a match we can stop searching.
impl<'a> Ord for Operator<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.name.cmp(&self.name)
    }
}

// Ord specifies that the ordering relationship defined by PartialOrd is total ordering.
impl<'a> PartialOrd for Operator<'a> {
    fn partial_cmp(&self, other: &Operator) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn oper_nop(nums: &[Number]) -> Result<Number, ExprError> {
    Ok (nums[0])
}

fn oper_add(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_add(rhs.integer).0;
    let float = lhs.float + rhs.float;
    Ok(Number { integer, float })
}

fn oper_sub(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_sub(rhs.integer).0;
    let float = lhs.float - rhs.float;
    Ok(Number { integer, float })
}

fn oper_unary_minus(nums: &[Number]) -> Result<Number, ExprError> {
    let rhs = nums[0];
    let integer = rhs.integer.overflowing_neg().0;
    let float = -rhs.float;
    Ok(Number { integer, float })
}

fn oper_logical_not(nums: &[Number]) -> Result<Number, ExprError> {
    let rhs = nums[0];
    let integer = (rhs.integer == 0) as u64;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_bit_not(nums: &[Number]) -> Result<Number, ExprError> {
    let rhs = nums[0];
    let integer = !rhs.integer;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_mul(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_mul(rhs.integer).0;
    let float = lhs.float * rhs.float;
    Ok(Number { integer, float })
}

fn oper_div(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_div(rhs.integer).0;
    let float = lhs.float / rhs.float;
    Ok(Number { integer, float })
}

fn oper_rem(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_rem(rhs.integer).0;
    let float = lhs.float % rhs.float;
    Ok(Number { integer, float })
}

fn oper_bit_lshift(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_shl(rhs.integer as u32).0;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_bit_rshift(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_shr(rhs.integer as u32).0;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_lt(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = (lhs.integer < rhs.integer) as u64;
    let float = (lhs.float < rhs.float) as u64 as f64;
    Ok(Number { integer, float })
}

fn oper_lte(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = (lhs.integer <= rhs.integer) as u64;
    let float = (lhs.float <= rhs.float) as u64 as f64;
    Ok(Number { integer, float })
}

fn oper_gt(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = (lhs.integer > rhs.integer) as u64;
    let float = (lhs.float > rhs.float) as u64 as f64;
    Ok(Number { integer, float })
}

fn oper_gte(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = (lhs.integer >= rhs.integer) as u64;
    let float = (lhs.float >= rhs.float) as u64 as f64;
    Ok(Number { integer, float })
}

fn oper_eq(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = (lhs.integer == rhs.integer) as u64;
    // I don't want to use a crate for this. Perhaps push this to some helper function.
    let abs_lhs = lhs.float.abs();
    let abs_rhs = rhs.float.abs();
    let abs_diff = (lhs.float - rhs.float).abs();
    let abs_cmp = if abs_lhs > abs_rhs {
        abs_rhs
    } else {
        abs_lhs
    };
    let float = (abs_diff <= abs_cmp * std::f64::EPSILON) as u64 as f64;
    Ok(Number { integer, float })
}

fn oper_ne(nums: &[Number]) -> Result<Number, ExprError> {
    let res = oper_eq(nums)?;
    let integer = !res.integer;
    Ok(Number { integer, float: integer as f64 })
}

fn oper_bit_and(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer & rhs.integer;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_bit_xor(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer ^ rhs.integer;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_bit_or(nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer | rhs.integer;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn func_dummy(_nums: &[Number]) -> Result<Number, ExprError> {
    Ok (Number { integer: 0u64, float: 0f64 })
}

fn func_sum(nums: &[Number]) -> Result<Number, ExprError> {
    let mut res = Number { integer: 0u64, float: 0f64 };
    for arg in nums  {
        res.integer += arg.integer;
        res.float += arg.float;
    }
    Ok(res)
}

fn func_avg(nums: &[Number]) -> Result<Number, ExprError> {
    let mut res = func_sum(nums)?;
    res.integer /= nums.len() as u64;
    res.float /= nums.len() as f64;
    Ok(res)
}

#[derive(Debug, Copy, Clone)]
struct NumberToken {
    idx_expr: usize,
    number: Number,
}

#[derive(Copy, Clone)]
struct OperatorToken {
    idx_expr: usize,
    idx_oper: usize,
}

impl fmt::Debug for OperatorToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.idx_oper < OPERATORS.len() {
            write!(f, "'{}'", OPERATORS[self.idx_oper].name)
        } else {
            write!(f, "Invalid Index {}", self.idx_oper)
        }
    }
}

#[derive(Copy, Clone)]
struct FunctionToken {
    idx_expr: usize,
    idx_func: usize,
    params: u8,
}

impl fmt::Debug for FunctionToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.idx_func < FUNCTIONS.len() {
            write!(f, "'{}'", &FUNCTIONS[self.idx_func].name)
        } else {
            write!(f, "Invalid Index {}", self.idx_func)
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Number(NumberToken),
    Operator(OperatorToken),
    Function(FunctionToken),
}

pub enum ExprResult {
    Number(Number),
    Command(String),
}

pub struct ExprCtx {
    queue_output: VecDeque<Token>,
    stack_op: Vec<Token>,
}

impl TryFrom<Token> for NumberToken {
    type Error = &'static str;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Number(num_token) => Ok(num_token),
            _ => Err("not a number token"),
        }
    }
}

impl TryFrom<Token> for OperatorToken {
    type Error = &'static str;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Operator(oper_token) => Ok(oper_token),
            _ => Err("not an operator token"),
        }
    }
}

impl TryFrom<Token> for FunctionToken {
    type Error = &'static str;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Function(func_token) => Ok(func_token),
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

    fn pop_move_to_output_queue(&mut self) {
        let token = self.stack_op.pop();
        debug_assert!(token.is_some());
        self.queue_output.push_back(token.unwrap());
    }

    fn push_to_output_queue(&mut self, token: Token, opt_prev_token: &mut Option<Token>) {
        self.queue_output.push_back(token);
        *opt_prev_token = Some(token);
    }

    fn push_to_op_stack(&mut self, token: Token, opt_prev_token: &mut Option<Token>) {
        self.stack_op.push(token);
        *opt_prev_token = Some(token);
    }

    fn verify_and_push_func_to_op_stack(&mut self, func_token: FunctionToken) -> Result<(), ExprError> {
        let func = &FUNCTIONS[func_token.idx_func];
        if func.params.contains(&func_token.params) {
            self.stack_op.push(Token::Function(func_token));
            Ok(())
        } else {
            // Too many or too few parameters passed to the function, bail.
            let message = format!("for function '{}'. expects [{}..{}) parameters, got {} instead",
                                  func.name, func.params.start, func.params.end, func_token.params);
            Err(ExprError { idx_expr: func_token.idx_expr,
                            kind: ExprErrorKind::InvalidParamCount,
                            message })
        }
    }

    fn pop_move_all_to_output_queue(&mut self) -> Result<(), ExprError> {
        while let Some(ref_token) = self.stack_op.last() {
            // If the stack has an open parenthesis, we have a parenthesis mismatch.
            match ref_token {
                Token::Operator(OperatorToken { idx_expr, idx_oper }) => {
                    debug_assert!(*idx_oper < OPERATORS.len());
                    let operator = &OPERATORS[*idx_oper];
                    if operator.kind == OperatorKind::OpenParen {
                        let message = format!("for opening parenthesis at {}", *idx_expr);
                        trace!("Parenthesis mismatch {}", message);
                        return Err(ExprError { idx_expr: *idx_expr,
                                               kind: ExprErrorKind::MismatchParenthesis,
                                               message });
                    } else {
                        self.pop_move_to_output_queue();
                    }
                }
                _ => self.pop_move_to_output_queue(),
            }
        }
        Ok(())
    }

    fn pop_func_from_stack(&mut self) -> Option<FunctionToken> {
        // If a function preceeds the open parenthesis, pop it to the output queue.
        if let Some(Token::Function(FunctionToken { idx_expr: _,
                                                    idx_func: _,
                                                    params: _ })) = self.stack_op.last() {
            // We safely unwrap both the token from the stack as well as the result from
            // the try_from() because we've already checked that the token on the top of
            // the stack is a function token.
            Some(FunctionToken::try_from(self.stack_op.pop().unwrap()).unwrap())
        } else {
            None
        }
    }

    fn collect_params(&mut self, params: usize, stack_output: &mut Vec<Number>) -> Option<Vec<Number>> {
        debug_assert!(params > 0);
        let mut parameters = Vec::with_capacity(params);
        for _ in 0..params {
            if let Some(param) = stack_output.pop() {
                parameters.push(param);
            } else {
                return None;
            }
        }
        // Reverse the parameters so left and right parameters are correct.
        parameters.reverse();
        Some(parameters)
    }

    fn process_parsed_operator(&mut self,
                               oper_token: OperatorToken,
                               opt_prev_token: &mut Option<Token>) -> Result<(), ExprError> {
        debug_assert!(oper_token.idx_oper < OPERATORS.len());
        let operator = &OPERATORS[oper_token.idx_oper];
        match operator.kind {
            OperatorKind::OpenParen => {
                // Previous token if any cannot be a close parenthesis or a number.
                // E.g "(5)(2)" or "5(2)".
                let missing_oper_or_func = match opt_prev_token {
                    Some(Token::Number(_)) => true,
                    Some(Token::Operator(
                        OperatorToken { idx_expr: _,
                                        idx_oper })) => OPERATORS[*idx_oper].kind == OperatorKind::CloseParen,
                    _ => false,
                };
                if missing_oper_or_func {
                    let message = format!("for open parenthesis at '{}'", oper_token.idx_expr);
                    trace!("{:?} {}", ExprErrorKind::MissingOperatorOrFunction, message);
                    return Err(ExprError { idx_expr: oper_token.idx_expr,
                                           kind: ExprErrorKind::MissingOperatorOrFunction,
                                           message });
                }
                self.push_to_op_stack(Token::Operator(oper_token), opt_prev_token);
            }

            OperatorKind::CloseParen => {
                // Find matching open parenthesis.
                let mut found_open_paren = false;
                while let Some(ref_token) = self.stack_op.last() {
                    match ref_token {
                        Token::Operator(OperatorToken { idx_expr: _,
                                                        idx_oper })
                                if OPERATORS[*idx_oper].kind == OperatorKind::OpenParen => {
                            found_open_paren = true;
                            break;
                        }
                        // Pop any other tokens to the output queue.
                        _ => self.pop_move_to_output_queue(),
                    }
                }

                if found_open_paren {
                    // Discard open parenthesis from the stack.
                    self.stack_op.pop().unwrap();

                    // Ensure close parenthesis is recorded as the previous token.
                    *opt_prev_token = Some(Token::Operator(oper_token));

                    // If a function preceeds the open parenthesis, increment its parameter count by 1.
                    // E.g "avg(5,6,7)". We've already incremented parameter count when there are more
                    // than one parameter when we handle the parameter separator operator. This is for
                    // the function's first parameter (left to right).
                    if let Some(mut func_token) = self.pop_func_from_stack() {
                        func_token.params += 1;
                        self.verify_and_push_func_to_op_stack(func_token)?;
                    }
                } else {
                    // If we didn't find a matching opening parenthesis, bail.
                    let message = format!("for closing parenthesis at {}", oper_token.idx_expr);
                    trace!("Parenthesis mismatch {}", message);
                    return Err(ExprError { idx_expr: oper_token.idx_expr,
                                           kind: ExprErrorKind::MismatchParenthesis,
                                           message });
                }
            }

            OperatorKind::ParamSep => {
                // Find the previous open parenthesis.
                while let Some(ref_token) = self.stack_op.last() {
                    match ref_token {
                        Token::Operator(OperatorToken { idx_expr: _, idx_oper })
                            if OPERATORS[*idx_oper].kind == OperatorKind::OpenParen => break,
                        _ => self.pop_move_to_output_queue(),
                    }
                }

                // If a token exists at the top of the op stack, it's an open parenthesis (due to the loop above).
                // This is debug asserted below for paranoia.
                if self.stack_op.last().is_some() {
                    let paren_token = self.stack_op.pop().unwrap();
                    #[cfg(debug_assertions)]
                    {
                        let oper_paren = OperatorToken::try_from(paren_token).unwrap();
                        debug_assert!(OPERATORS[oper_paren.idx_oper].kind == OperatorKind::OpenParen);
                    }

                    // If a function preceeds the open parenthesis, increment its parameter count by 1
                    // and re-push the function and the previously popped open parenthesis back to the
                    // op stack. It is important we do -NOT- update "opt_prev_token" while doing this
                    // temporary modification of a token's data in stack.
                    if let Some(mut func_token) = self.pop_func_from_stack() {
                        func_token.params += 1;
                        self.stack_op.push(Token::Function(func_token));
                        self.stack_op.push(paren_token);
                    } else {
                        // No function preceeding open parenthesis for a parameter separator. E.g "(32,5)"
                        let message = format!("for parameter separator '{}' at {}", operator.name, oper_token.idx_expr);
                        trace!("{:?} {}", ExprErrorKind::MissingFunction, message);
                        return Err(ExprError { idx_expr: 0,
                                               kind: ExprErrorKind::MissingFunction,
                                               message });
                    }
                } else {
                    // No matching open parenthesis for the parameter separator. E.g "32,4".
                    let message = format!("for parameter separator '{}' at {}", operator.name, oper_token.idx_expr);
                    trace!("{:?} {}", ExprErrorKind::MissingParenthesis, message);
                    return Err(ExprError { idx_expr: 0,
                                           kind: ExprErrorKind::MissingParenthesis,
                                           message });
                }
            }

            _ => {
                while let Some(ref_token) = self.stack_op.last() {
                    match ref_token {
                        Token::Operator(OperatorToken { idx_expr: _, idx_oper }) => {
                            let token_stack_oper = &OPERATORS[*idx_oper];
                            debug_assert!(token_stack_oper.kind != OperatorKind::CloseParen);
                            if token_stack_oper.kind == OperatorKind::OpenParen {
                                break;
                            }

                            // Pop operator with higher priority (depending on associativity) to the output queue.
                            if    token_stack_oper.prec < operator.prec
                               || operator.assoc == OperatorAssoc::Left && operator.prec == token_stack_oper.prec {
                                self.pop_move_to_output_queue();
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }

                self.push_to_op_stack(Token::Operator(oper_token), opt_prev_token);
            }
        }

        Ok(())
    }
}

fn parse_function(str_expr: &str, functions: &[Function]) -> Option<usize> {
    debug_assert_eq!(str_expr.trim_start_matches(char::is_whitespace), str_expr);

    // All functions must be succeeded by an open parenthesis.
    // Collect function name till we find an open parenthesis and then check if that function
    // exists in the function table.
    let mut is_found = false;
    let mut idx_found = 0;
    for (idx, func) in functions.iter().enumerate() {
        if str_expr.starts_with(func.name) {
            idx_found = idx;
            is_found = true;
            break;
        }
    }

    if is_found {
        trace!("found {}", &functions[idx_found].help);
        Some(idx_found)
    } else {
        None
    }
}

fn parse_number(str_expr: &str) -> (Option<Number>, usize) {
    debug_assert_eq!(str_expr.trim_start_matches(char::is_whitespace), str_expr);

    let mut radix: u32 = 10;
    let mut len_prefix = 0;
    let mut iter_expr = str_expr.chars().peekable();

    // Parse any prefix that is explicitly part of the given expression
    if str_expr.starts_with('0') {
        len_prefix += 1;
        iter_expr.next();
        if let Some(n) = iter_expr.peek() {
            match n {
                'x' | 'X' => { len_prefix += 1; iter_expr.next(); radix = 16; }
                'n' | 'N' => { len_prefix += 1; iter_expr.next(); radix = 2; }
                'o' | 'O' => { len_prefix += 1; iter_expr.next(); radix = 8; }
                _ => (),
            }
        } else {
            return (Some(Number { integer: 0u64, float: 0f64 }), 1)
        }
    }

    // Rust string to number conversion functions do not grok prefixes (e.g., "0xf" will have
    // to be fed to it as just "f"). So we aggregate the number into 'str_num' and keep track
    // of the length of any prefix that's already part of the expression in 'len_prefix' (as
    // done above). This also has a side effect in making the loop below faster as we eliminate
    // checks that doesn't need to happen on every iteration.
    let mut str_num = String::with_capacity(64);
    let mut has_dec_pt = false;
    let mut is_fp_exp_notation = false;

    debug_assert!(radix != 0);
    for (idx, chr) in iter_expr.enumerate() {
        // Make sure we are not in the middle of a UTF-8 sequence.
        debug_assert!(str_expr.is_char_boundary(idx));

        if chr.is_whitespace() {
            continue;
        }

        if chr.is_digit(radix) {
            // Valid digit for the radix.
            str_num.push(chr);
        } else if chr == '.' && radix == 10 && !has_dec_pt {
            // Valid decimal point for decimal number and is the first decimal point.
            has_dec_pt = true;
            str_num.push(chr);
        } else if has_dec_pt && (chr == 'e' || chr == 'E') {
            // Floating point exponent notation ("2.5e10" or "2.5E-10").
            str_num.push(chr);
            is_fp_exp_notation = true;
        } else if is_fp_exp_notation && (chr == '+' || chr == '-') {
            // FP exponent notation +/- power-of character.
            str_num.push(chr);
        } else {
            break;
        }
    }

    // If we have no digits or if the number ends in a decimal point, it's not a valid.
    if str_num.is_empty() || str_num.ends_with('.') {
        return (None, 0)
    }

    // 'consumed' contains the length of characters consumed by parsing this number.
    let consumed = len_prefix + str_num.len();

    if !has_dec_pt {
        // Parse as integer.
        match u64::from_str_radix(&str_num, radix) {
            Ok(v) => (Some(Number { integer: v, float: v as f64 }), consumed),
            _ => (None, 0),
        }
    } else {
        // Parse as float.
        // If the float is (+/-)Inf/NaN or otherwise not representable in a u64, casting it
        // results in 0. Right now, I don't know a fool proof way of determining this.
        // Do it later.
        // TODO: We might also want to consider aborting parsing here in the Inf/NaN case.
        use std::str::FromStr;
        match f64::from_str(&str_num) {
            Ok(v) => (Some(Number { integer: v as u64, float: v }), consumed),
            _ => (None, 0),
        }
    }
}

fn parse_operator(str_expr: &str, operators: &[Operator], opt_prev_token: &mut Option<Token>) -> Option<usize> {
    debug_assert_eq!(str_expr.trim_start_matches(char::is_whitespace), str_expr);

    let mut is_found = false;
    let mut idx_found = 0;

    for (idx, op) in operators.iter().enumerate() {
        // If this is the first occurrence of this operator, record where we found it.
        // Otherwise, record the currently found operator only if its length exceeds that
        // of a previously found (i.e., we should be able to find "<<" and not stop at "<"),
        if str_expr.starts_with(op.name)
           && (!is_found
                || op.name.len() > operators[idx_found].name.len()) {
            // Is this a left associative operator, ensure a previous token exists and that
            // it's not an operator (other than close parenthesis), otherwise skip finding
            // it as a valid operator.
            if op.assoc == OperatorAssoc::Left {
                match opt_prev_token {
                    // E.g. "-4" or ",5".
                    None => continue,
                    Some(Token::Operator(OperatorToken { idx_expr: _, idx_oper })) => {
                        debug_assert!(*idx_oper < operators.len());
                        // E.g. ")-5" when parsing "-" can be valid. So don't skip finding "-".
                        // E.g. "(<<7" when parsing "<<" is invalid, so skip finding it.
                        if operators[*idx_oper].kind != OperatorKind::CloseParen
                            && (operators[*idx_oper].assoc != OperatorAssoc::Left
                                 || operators[*idx_oper].params != 1) {
                            continue;
                        }
                    }
                    _ => (),
                }
            }
            // If this is a right associative operator, ensure if a previous token exists
            // that it's not a right associative unary operator. If it is, it's a malformed
            // expression like "2+++4". Note: "2++4" is 2+(+4), i.e. 2 plus unary plus 4 which is valid.
            // NOTE: I've got rid of post/pre inc/dec. but this does handle the case if I add it back.
            else if op.assoc == OperatorAssoc::Right {
                match opt_prev_token {
                    None => (),
                    // E.g. "++4" when previous token was a unary "+" operator.
                    Some(Token::Operator(OperatorToken { idx_expr: _, idx_oper })) => {
                        if operators[*idx_oper].assoc == OperatorAssoc::Right && operators[*idx_oper].params == 1 {
                            is_found = false;
                            break;
                        }
                    }
                    _ => (),
                }
            }
            is_found = true;
            idx_found = idx;
        }
    }

    if is_found {
        trace!("found '{}' - {}", operators[idx_found].name, operators[idx_found].help);
        Some(idx_found)
    } else {
        None
    }
}

fn verify_prev_token_not_function(opt_prev_token: &Option<Token>) -> Result<(), ExprError> {
    match opt_prev_token {
        Some(Token::Function(FunctionToken { idx_expr, idx_func, params: _ })) => {
            let idx_open_paren = idx_expr + FUNCTIONS[*idx_func].name.len();
            let message = format!("at {} for function '{}'", idx_open_paren, &FUNCTIONS[*idx_func].name);
            trace!("{:?} {}", ExprErrorKind::MissingParenthesis, message);
            Err(ExprError { idx_expr: idx_open_paren,
                            kind: ExprErrorKind::MissingParenthesis,
                            message })
        }
        _ => Ok(())
    }
}

fn verify_prev_token_not_close_paren(opt_prev_token: &Option<Token>) -> Result<(), ExprError> {
    match opt_prev_token {
        Some(Token::Operator(
            OperatorToken { idx_expr, idx_oper })) if OPERATORS[*idx_oper].kind == OperatorKind::CloseParen => {
            let idx_oper_or_func = idx_expr + OPERATORS[*idx_oper].name.len();
            let message = format!("at {}", idx_oper_or_func);
            trace!("{:?} {}", ExprErrorKind::MissingOperatorOrFunction, message);
            Err(ExprError { idx_expr: idx_oper_or_func,
                            kind: ExprErrorKind::MissingOperatorOrFunction,
                            message })
        }
        _ => Ok(())
    }
}


pub fn parse(str_expr: &str) -> Result<ExprCtx, ExprError> {
    // We iterate by characters here because we want to know the index of every token.
    // The index is primarily for reporting parsing and evaluation errors.
    // If we didn't need to store the index, we can easily loop, trim_start whitespaces,
    // and just re-assign 'str_subexpr' to the string slice given by parse_number().
    let mut len_token = 0;
    let mut expr_ctx = ExprCtx::new();
    let mut opt_prev_token: Option<Token> = None;

    for (idx, chr) in str_expr.chars().enumerate() {
        // Make sure we are not in the middle of a UTF-8 sequence.
        debug_assert!(str_expr.is_char_boundary(idx));
        if chr.is_whitespace() {
            continue;
        }
        if len_token > 1 {
            len_token -= 1;
            continue;
        }
        let str_subexpr = &str_expr[idx..];
        if let (Some(number), len_str) = parse_number(str_subexpr) {
            // If the previous token was a function or a close parenthesis, it's invalid.
            // E.g "avg 32.5" or "(2)3" or "(1).5".
            verify_prev_token_not_function(&opt_prev_token)?;
            verify_prev_token_not_close_paren(&opt_prev_token)?;
            trace!("number  : {} (0x{:x})", number.integer, number.integer);
            len_token = len_str;
            let num_token = NumberToken { idx_expr: idx, number };
            expr_ctx.push_to_output_queue(Token::Number(num_token), &mut opt_prev_token);
        } else if let Some(idx_oper) = parse_operator(str_subexpr, &OPERATORS, &mut opt_prev_token) {
            debug_assert!(idx_oper < OPERATORS.len());
            // If the previous token was a function, this must be an open parenthesis.
            // E.g "avg +"; otherwise this is an invalid expression.
            if let Some(Token::Function(FunctionToken { idx_expr: idx_expr_func, idx_func, params: _ })) = opt_prev_token {
                // Calculate where the open parenthesis must appear, we don't use "idx" because
                // it includes all the whitespace after the function name. We want to report the
                // character immediately after the name of the function.
                // E.g we want position X in "avgX Y+" rather than position Y.
                let idx_open_paren = idx_expr_func + FUNCTIONS[idx_func].name.len();
                if OPERATORS[idx_oper].kind != OperatorKind::OpenParen {
                    let message = format!("at {} for function '{}'", idx_open_paren, &FUNCTIONS[idx_func].name);
                    trace!("{:?} {}", ExprErrorKind::MissingParenthesis, message);
                    return Err(ExprError { idx_expr: idx_open_paren,
                                           kind: ExprErrorKind::MissingParenthesis,
                                           message });
                }
            }
            trace!("operator: {}", &OPERATORS[idx_oper].name);
            len_token = OPERATORS[idx_oper].name.len();
            let oper_token = OperatorToken { idx_expr: idx, idx_oper };
            expr_ctx.process_parsed_operator(oper_token, &mut opt_prev_token)?;
        } else if let Some(idx_func) = parse_function(str_subexpr, &FUNCTIONS) {
            debug_assert!(idx_func < FUNCTIONS.len());
            // If the previous token was a function, we have an invalid expression.
            // E.g "avg avg"; functions must be followed by open parenthesis only.
            verify_prev_token_not_function(&opt_prev_token)?;
            trace!("function: {}", &FUNCTIONS[idx_func].name);
            len_token = FUNCTIONS[idx_func].name.len();
            let func_token = FunctionToken { idx_expr: idx, idx_func, params: 0 };
            expr_ctx.push_to_op_stack(Token::Function(func_token), &mut opt_prev_token);
        } else {
            let message = format!("at {}", idx);
            trace!("{:?} {}", ExprErrorKind::InvalidExpr, message);
            return Err(ExprError { idx_expr: idx,
                                    kind: ExprErrorKind::InvalidExpr,
                                    message });
        }
    }

    // If the last parsed token was a function, that's an invalid expression.
    // E.g "23 + avg".
    verify_prev_token_not_function(&opt_prev_token)?;

    if expr_ctx.stack_op.is_empty() && expr_ctx.queue_output.is_empty() {
        trace!("'{:?}", ExprErrorKind::EmptyExpr);
        return Err(ExprError { idx_expr: 0,
                               kind: ExprErrorKind::EmptyExpr,
                               message: "".to_string() });
    }

    debug!("Op Stack:");
    for (idx,token) in expr_ctx.stack_op.iter().rev().enumerate() {
        debug!("  stack[{}]: {:?}", expr_ctx.stack_op.len() - 1 - idx, token);
    }
    debug!("Output Queue:");
    for (idx,token) in expr_ctx.queue_output.iter().enumerate() {
        debug!("  queue[{}]: {:?}", idx, token);
    }

    // Pop and move remaining tokens from op stack to the output queue.
    expr_ctx.pop_move_all_to_output_queue()?;

    Ok(expr_ctx)
}

pub fn evaluate(expr_ctx: &mut ExprCtx) -> Result<ExprResult, ExprError> {
    // Pop tokens from the output queue to an output stack and process them.
    let mut stack_output: Vec<Number> = Vec::with_capacity(PRE_ALLOC_TOKENS);
    while let Some(token) = expr_ctx.queue_output.pop_front() {
        match token {
            Token::Number(NumberToken { idx_expr: _, number }) => {
                stack_output.push(number);
            }

            Token::Operator(OperatorToken { idx_expr, idx_oper }) => {
                debug_assert!(idx_oper < OPERATORS.len());
                let operator = &OPERATORS[idx_oper];
                if let Some(parameters) = expr_ctx.collect_params(operator.params as usize, &mut stack_output) {
                    debug_assert!(parameters.len() == operator.params as usize);
                    let res_expr = (operator.func)(&parameters)?;
                    stack_output.push(res_expr);
                } else {
                    let message = format!("for operator '{}' at {}", operator.name, idx_expr);
                    trace!("{:?} {}", ExprErrorKind::InvalidParamCount, message);
                    return Err(ExprError { idx_expr,
                                           kind: ExprErrorKind::InvalidParamCount,
                                           message });
                }
            }

            Token::Function(FunctionToken { idx_expr, idx_func, params }) => {
                debug_assert!(idx_func < FUNCTIONS.len());
                let function = &FUNCTIONS[idx_func];
                if let Some(parameters) = expr_ctx.collect_params(params as usize, &mut stack_output) {
                    debug_assert!(parameters.len() == params as usize);
                    let res_expr = (function.func)(&parameters)?;
                    stack_output.push(res_expr);
                } else {
                    let message = format!("for function '{}' at {}", function.name, idx_expr);
                    trace!("{:?} {}", ExprErrorKind::InvalidParamCount, message);
                    return Err(ExprError { idx_expr,
                                           kind: ExprErrorKind::InvalidParamCount,
                                           message });
                }
            }
        }
    }

    if let Some(token) = stack_output.pop() {
        Ok(ExprResult::Number(token))
    } else {
        let message = "evaluation failed".to_string();
        trace!("{}", message);
        Err(ExprError { idx_expr: 0,
                        kind: ExprErrorKind::InvalidExpr,
                        message })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_invalid_numbers() {
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
                            "..5",
                            "2.5ee4",
                            "2.5e++4",
                            "2.5ee++4",
                            "2.5e--5"
        ];
        // Make sure we never parse operators as valid numbers.
        for i in 0..OPERATORS.len() {
            vec_nums.push(&OPERATORS[i].name);
        }
        // Make sure we never parse functions as valid numbers.
        for i in 0..FUNCTIONS.len() {
            vec_nums.push(&FUNCTIONS[i].name);
        }
        for num_res in vec_nums {
            let (number, len_str) = parse_number(num_res);
            assert!(number.is_none());
            assert_eq!(len_str, 0);
        }
    }

    #[test]
    fn parse_valid_number_u64() {
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
            ("0n0",  0  ), ("0n1",  1  ), ("0n10", 2  ), ("0n11", 3  ), ("0n100", 4 ),
            ("0n11111111111111111111111111111111", 0xffffffff),
            ("0n1111111111111111111111111111111111111111111111111111111111111111", 0xffffffffffffffff),
            ("0n0000000000000000000000000000000011111111111111111111111111111111", 0xffffffff),
            ("0n1111111111111111111111111111111100000000000000000000000000000000", 0xffffffff00000000),
            // Octal prefix.
            ("0o0",  0  ), ("0o1",  1  ), ("0o2",  2  ), ("0o3",  3  ), ("0o4",  4  ),
            ("0o5",  5  ), ("0o6",  6  ), ("0o7",  7  ), ("0o7",  7  ),
            ("0o10", 8  ), ("0o11", 9  ),
            ("0o77", 63 ), ("0o100", 64),
        ];
        for int_res in pair_int_result {
            let (number, len_str) = parse_number(int_res.0);
            assert!(number.is_some(), "failed for ('{}', {})", int_res.0, int_res.1);
            assert_eq!(number.unwrap().integer, int_res.1);
            assert_eq!(len_str, int_res.0.len());
        }
    }

    #[test]
    fn parse_valid_number_f64() {
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
            let (number, len_str) = parse_number(float_res.0);
            assert!(number.is_some(), "failed for ('{}', {})", float_res.0, float_res.1);
            assert_eq!(number.unwrap().float, float_res.1);
            assert_eq!(len_str, float_res.0.len());
        }
    }

    #[test]
    fn is_operator_table_valid() {
        let mut open_paren_count = 0;
        let mut close_paren_count = 0;
        let mut var_assign_count = 0;
        let mut param_sep_count = 0;
        for (idx, oper) in OPERATORS.iter().enumerate() {
            assert!(oper.params < 3, "Operator '{}' at {} has {} parameters. \
                    Operators can have at most 2 parameters.", oper.name, idx, oper.params);
            assert!(oper.kind != OperatorKind::Regular || oper.params > 0,
                    "Regular operator '{}' at {} cannot have 0 parameters.", oper.name, idx);

            assert_eq!(oper.name.chars().all(|x| x.is_digit(10)), false,
                       "Operator '{}' invalid. Name cannot contain digits.", oper.name);
            assert_eq!(oper.name.chars().all(|x| x == '_'), false,
                       "Operator '{}' invalid. Name cannot contain '_' character.", oper.name);

            // Ensure open and close parenthesis operators have Nil associativity.
            match oper.kind {
                OperatorKind::OpenParen => {
                    assert_eq!(oper.assoc, OperatorAssoc::Nil,
                            "Open parenthesis operator '{}' at {} must have no associativity.", oper.name, idx);
                    open_paren_count += 1;
                }
                OperatorKind::CloseParen => {
                    assert_eq!(oper.assoc, OperatorAssoc::Nil,
                            "Close parenthesis operator '{}' at {} must have no associativity.", oper.name, idx);
                    close_paren_count += 1;
                }
                OperatorKind::VarAssign => var_assign_count += 1,
                OperatorKind::ParamSep => param_sep_count += 1,
                _ => (),
            }

            for (idxcmp, opercmp) in OPERATORS.iter().enumerate() {
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
        assert_eq!(var_assign_count, 1);
        assert_eq!(param_sep_count, 1);
    }

    #[test]
    fn is_function_table_valid() {
        for (idx, func) in FUNCTIONS.iter().enumerate() {
            assert!(!func.params.contains(&MAX_FN_PARAMS),
                    "Function '{}' at {} exceeds maximum parameters of {}. Use/alter the maximum.",
                    func.name, idx, MAX_FN_PARAMS);

            assert_eq!(func.name.is_empty(), false,
                    "Function at {} invalid. Name cannot be 0 length.", idx);
            assert_eq!(func.name.chars().nth(0).unwrap().is_digit(10), false,
                       "Function '{}' invalid. Name cannot start with digits.", func.name);
            assert_ne!(func.name.chars().nth(0).unwrap(), '_',
                       "Function '{}' invalid. Name cannot start with an '_' character.", func.name);

            // Ensure no duplicate functions names.
            for (idxcmp, funccmp) in FUNCTIONS.iter().enumerate() {
                if idxcmp != idx {
                    assert!(func.name != funccmp.name,
                            "Duplicate function '{}' at {} and {}", func.name, idx, idxcmp);
                }
            }
        }
    }
}

