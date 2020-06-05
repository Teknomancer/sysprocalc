use std::fmt;
use std::ops::Range;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::convert::TryFrom;
use log::{trace, debug};   // others: {warn,info}

// Number of tokens to pre-allocate per ExprCtx.
const PRE_ALLOC_TOKENS: usize = 16;

static OPERS: [Oper<'static>; 26] = [
    // Precedence 1 (highest priority)
    Oper { kind: OperKind::OpenParen,  prec: 1,  params: 0, assoc: OperAssoc::Nil,   name: "(",  syntax: "(<expr>",            help: "Begin expression."             , evalfn: oper_nop },
    Oper { kind: OperKind::CloseParen, prec: 1,  params: 0, assoc: OperAssoc::Nil,   name: ")",  syntax: "<expr>)",            help: "End expression."               , evalfn: oper_nop },
    // Precendence 4 (appears in array before 2 because of parsing logic with unary operators)
    Oper { kind: OperKind::Regular,    prec: 4,  params: 2, assoc: OperAssoc::Left,  name: "+",  syntax: "<expr> + <expr>",    help: "Addition."                     , evalfn: oper_add },
    Oper { kind: OperKind::Regular,    prec: 4,  params: 2, assoc: OperAssoc::Left,  name: "-",  syntax: "<expr> - <expr>",    help: "Subtraction."                  , evalfn: oper_sub },
    // Precedence 2
    Oper { kind: OperKind::Regular,    prec: 2,  params: 1, assoc: OperAssoc::Right, name: "+",  syntax: "+<expr>",            help: "Unary plus."                   , evalfn: oper_nop },
    Oper { kind: OperKind::Regular,    prec: 2,  params: 1, assoc: OperAssoc::Right, name: "-",  syntax: "-<expr>",            help: "Unary minus."                  , evalfn: oper_unary_minus },
    Oper { kind: OperKind::Regular,    prec: 2,  params: 1, assoc: OperAssoc::Right, name: "!",  syntax: "!<expr>",            help: "Logical NOT."                  , evalfn: oper_logical_not },
    Oper { kind: OperKind::Regular,    prec: 2,  params: 1, assoc: OperAssoc::Right, name: "~",  syntax: "~<expr>",            help: "Bitwise NOT."                  , evalfn: oper_bit_not },
    // Precedence 3
    Oper { kind: OperKind::Regular,    prec: 3,  params: 2, assoc: OperAssoc::Left,  name: "*",  syntax: "<expr> * <expr>",    help: "Multiplication."               , evalfn: oper_mul },
    Oper { kind: OperKind::Regular,    prec: 3,  params: 2, assoc: OperAssoc::Left,  name: "/",  syntax: "<expr> / <expr>",    help: "Division."                     , evalfn: oper_div },
    Oper { kind: OperKind::Regular,    prec: 3,  params: 2, assoc: OperAssoc::Left,  name: "%",  syntax: "<expr> % <expr>",    help: "Remainder."                    , evalfn: oper_rem },
    // Precedence 5
    Oper { kind: OperKind::Regular,    prec: 5,  params: 2, assoc: OperAssoc::Left,  name: "<<", syntax: "<expr> << <expr>",   help: "Bitwise left-shift."           , evalfn: oper_bit_lshift },
    Oper { kind: OperKind::Regular,    prec: 5,  params: 2, assoc: OperAssoc::Left,  name: ">>", syntax: "<expr> >> <expr>",   help: "Bitwise right-shift."          , evalfn: oper_bit_rshift },
    // Precedence 6
    Oper { kind: OperKind::Regular,    prec: 6,  params: 2, assoc: OperAssoc::Left,  name: "<",  syntax: "<expr> < <expr>",    help: "Less-than."                    , evalfn: oper_lt },
    Oper { kind: OperKind::Regular,    prec: 6,  params: 2, assoc: OperAssoc::Left,  name: "<=", syntax: "<expr> <= <expr>",   help: "Less-than-or-equals."          , evalfn: oper_lte },
    Oper { kind: OperKind::Regular,    prec: 6,  params: 2, assoc: OperAssoc::Left,  name: ">",  syntax: "<expr> > <expr>",    help: "Greater-than."                 , evalfn: oper_gt },
    Oper { kind: OperKind::Regular,    prec: 6,  params: 2, assoc: OperAssoc::Left,  name: ">=", syntax: "<expr> >= <expr>",   help: "Greater-than-or-equals."       , evalfn: oper_gte },
    // Precedence 7
    Oper { kind: OperKind::Regular,    prec: 7,  params: 2, assoc: OperAssoc::Left,  name: "==", syntax: "<expr> == <expr>",   help: "Equals."                       , evalfn: oper_eq },
    Oper { kind: OperKind::Regular,    prec: 7,  params: 2, assoc: OperAssoc::Left,  name: "!=", syntax: "<expr> != <expr>",   help: "Not-equals."                   , evalfn: oper_ne },
    // Precedence 8
    Oper { kind: OperKind::Regular,    prec: 8,  params: 2, assoc: OperAssoc::Left,  name: "&",  syntax: "<expr> & <expr>",    help: "Bitwise AND."                  , evalfn: oper_bit_and },
    // Precedence 9
    Oper { kind: OperKind::Regular,    prec: 9,  params: 2, assoc: OperAssoc::Left,  name: "^",  syntax: "<expr> ^ <expr>",    help: "Bitwise XOR."                  , evalfn: oper_bit_xor },
    // Precedence 10
    Oper { kind: OperKind::Regular,    prec: 10, params: 2, assoc: OperAssoc::Left,  name: "|",  syntax: "<expr> | <expr>",    help: "Bitwise OR."                   , evalfn: oper_bit_or },
    // Precedence 11
    Oper { kind: OperKind::Regular,    prec: 11, params: 2, assoc: OperAssoc::Left,  name: "&&", syntax: "<expr> && <expr>",   help: "Logical AND."                  , evalfn: oper_nop },
    // Precedence 12
    Oper { kind: OperKind::Regular,    prec: 12, params: 2, assoc: OperAssoc::Left,  name: "||", syntax: "<expr> || <expr>",   help: "Logical OR."                   , evalfn: oper_nop },
    // Precedence 13
    Oper { kind: OperKind::VarAssign,  prec: 13, params: 2, assoc: OperAssoc::Left,  name: "=",  syntax: "<var> = <expr>",     help: "Variable assignment."          , evalfn: oper_nop },
    // Precedence 14
    Oper { kind: OperKind::ParamSep,   prec: 14, params: 2, assoc: OperAssoc::Left,  name: ",",  syntax: "<param1>, <param2>", help: "Function parameter separator." , evalfn: oper_nop },
];

const MAX_FN_PARAMS: u8 = u8::max_value();
static FUNCS: [Func<'static>; 3] = [
    Func {
        name: "sum",
        params: Range { start: 2, end: MAX_FN_PARAMS },
        syntax: "<n1>,<n2>[,<n3>...<nX>]",
        help: "Sum",
        evalfn: func_sum,
    },
    Func {
        name: "avg",
        params: Range { start: 2, end: MAX_FN_PARAMS },
        syntax: "<n1>,<n2>[,<n3>...<nX>]",
        help: "Average",
        evalfn: func_avg,
    },
    Func {
        name: "if",
        params: Range { start: 3, end: 4 },
        syntax: "<cond>,<n1>,<n2>",
        help: "If <cond> is true, returns <n1> else <n2>",
        evalfn: func_dummy,
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
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
    MissingOperatorOrFunction,
    MissingParenthesis,
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
            ExprErrorKind::FailedEvaluation => "evaluation failed",
            ExprErrorKind::InvalidExpr => "invalid character",
            ExprErrorKind::InvalidParamCount => "incorrect number of parameters",
            ExprErrorKind::InvalidParamType => "invalid parameter type",
            ExprErrorKind::MismatchParenthesis => "parenthesis mismatch",
            ExprErrorKind::MissingFunction => "function missing",
            ExprErrorKind::MissingOperand => "operand missing",
            ExprErrorKind::MissingOperatorOrFunction => "operator or function missing",
            ExprErrorKind::MissingParenthesis => "parenthesis missing",
        };
        write!(f, "{} {}", str_errkind, self.message)
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Number {
    pub integer: u64,
    pub float: f64,
}

type PfnOper = fn(idx_expr: usize, &[Number]) -> Result<Number, ExprError>;
type PfnFunc = fn(idx_expr: usize, &[Number]) -> Result<Number, ExprError>;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum OperAssoc {
    Nil,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum OperKind {
    Regular,
    OpenParen,
    CloseParen,
    ParamSep,
    VarAssign,
}

struct Oper<'a> {
    kind: OperKind,
    prec: u8,
    params: u8,
    assoc: OperAssoc,
    name: &'a str,
    syntax: &'a str,
    help: &'a str,
    evalfn: PfnOper,
}

struct Func<'a> {
    name: &'a str,
    params: Range<u8>,
    syntax: &'a str,
    help: &'a str,
    evalfn: PfnFunc,
}

// Eq specifies that the equality relationship defined by PartialEq is a total equality.
impl<'a> Eq for Oper<'a> {}

// PartialEq is required by PartialOrd which is required for sorting.
impl<'a> PartialEq for Oper<'a> {
    fn eq(&self, other: &Oper) -> bool {
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
impl<'a> Ord for Oper<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.name.cmp(&self.name)
    }
}

// Ord specifies that the ordering relationship defined by PartialOrd is total ordering.
impl<'a> PartialOrd for Oper<'a> {
    fn partial_cmp(&self, other: &Oper) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cmp_eq_f64(a: f64, b: f64) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let abs_diff = (a - b).abs();
    let abs_cmp = if abs_a > abs_b {
        abs_b
    } else {
        abs_a
    };

    abs_diff <= abs_cmp * std::f64::EPSILON
}

fn oper_nop(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    Ok (nums[0])
}

fn oper_add(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_add(rhs.integer).0;
    let float = lhs.float + rhs.float;
    Ok(Number { integer, float })
}

fn oper_sub(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_sub(rhs.integer).0;
    let float = lhs.float - rhs.float;
    Ok(Number { integer, float })
}

fn oper_unary_minus(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let rhs = nums[0];
    let integer = rhs.integer.overflowing_neg().0;
    let float = -rhs.float;
    Ok(Number { integer, float })
}

fn oper_logical_not(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let rhs = nums[0];
    let integer = (rhs.integer == 0) as u64;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_bit_not(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let rhs = nums[0];
    let integer = !rhs.integer;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_mul(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_mul(rhs.integer).0;
    let float = lhs.float * rhs.float;
    Ok(Number { integer, float })
}

fn oper_div(idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    if  rhs.integer != 0 && !cmp_eq_f64(rhs.float, 0f64) {
        let integer = lhs.integer.overflowing_div(rhs.integer).0;
        let float = lhs.float / rhs.float;
        Ok(Number { integer, float })
    } else {
        let message = format!("due to division by 0 for operator at {}", idx_expr);
        Err(ExprError { idx_expr, kind: ExprErrorKind::FailedEvaluation, message })
    }
}

fn oper_rem(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_rem(rhs.integer).0;
    let float = lhs.float % rhs.float;
    Ok(Number { integer, float })
}

fn oper_bit_lshift(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_shl(rhs.integer as u32).0;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_bit_rshift(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.overflowing_shr(rhs.integer as u32).0;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_lt(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = (lhs.integer < rhs.integer) as u64;
    let float = (lhs.float < rhs.float) as u64 as f64;
    Ok(Number { integer, float })
}

fn oper_lte(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = (lhs.integer <= rhs.integer) as u64;
    let float = (lhs.float <= rhs.float) as u64 as f64;
    Ok(Number { integer, float })
}

fn oper_gt(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = (lhs.integer > rhs.integer) as u64;
    let float = (lhs.float > rhs.float) as u64 as f64;
    Ok(Number { integer, float })
}

fn oper_gte(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = (lhs.integer >= rhs.integer) as u64;
    let float = (lhs.float >= rhs.float) as u64 as f64;
    Ok(Number { integer, float })
}

fn oper_eq(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = (lhs.integer == rhs.integer) as u64;
    let float = cmp_eq_f64(lhs.float, rhs.float) as u64 as f64;
    Ok(Number { integer, float })
}

fn oper_ne(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = (lhs.integer != rhs.integer) as u64;
    let float = !cmp_eq_f64(lhs.float, rhs.float) as u64 as f64;
    Ok(Number { integer, float })
}

fn oper_bit_and(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer & rhs.integer;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_bit_xor(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer ^ rhs.integer;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_bit_or(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer | rhs.integer;
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn func_dummy(_idx_expr: usize, _nums: &[Number]) -> Result<Number, ExprError> {
    Ok (Number { integer: 0u64, float: 0f64 })
}

fn func_sum(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let mut res = Number { integer: 0u64, float: 0f64 };
    for arg in nums  {
        res.integer += arg.integer;
        res.float += arg.float;
    }
    Ok(res)
}

fn func_avg(idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let mut res = func_sum(idx_expr, nums)?;
    res.integer /= nums.len() as u64;
    res.float /= nums.len() as f64;
    Ok(res)
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

pub enum ExprResult {
    Number(Number),
    Command(String),
}

pub struct ExprCtx {
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

    fn verify_and_push_func_to_op_stack(&mut self, func_token: FuncToken) -> Result<(), ExprError> {
        let func = &FUNCS[func_token.idx_func];
        if func.params.contains(&func_token.params) {
            self.stack_op.push(Token::Func(func_token));
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
                Token::Oper(OperToken { idx_oper, idx_expr }) => {
                    debug_assert!(*idx_oper < OPERS.len());
                    let oper = &OPERS[*idx_oper];
                    if oper.kind == OperKind::OpenParen {
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

    fn pop_func_from_stack(&mut self) -> Option<FuncToken> {
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

    fn process_parsed_oper(&mut self,
                           oper_token: OperToken,
                           opt_prev_token: &mut Option<Token>) -> Result<(), ExprError> {
        debug_assert!(oper_token.idx_oper < OPERS.len());
        let oper = &OPERS[oper_token.idx_oper];
        match oper.kind {
            OperKind::OpenParen => {
                // Previous token if any cannot be a close parenthesis or a number.
                // E.g "(5)(2)" or "5(2)".
                let missing_oper_or_func = match opt_prev_token {
                    Some(Token::Num(_)) => true,
                    Some(Token::Oper(OperToken { idx_oper, .. })) => {
                        OPERS[*idx_oper].kind == OperKind::CloseParen
                    }
                    _ => false,
                };
                if missing_oper_or_func {
                    let message = format!("for open parenthesis at '{}'", oper_token.idx_expr);
                    trace!("{:?} {}", ExprErrorKind::MissingOperatorOrFunction, message);
                    return Err(ExprError { idx_expr: oper_token.idx_expr,
                                           kind: ExprErrorKind::MissingOperatorOrFunction,
                                           message });
                }
                self.push_to_op_stack(Token::Oper(oper_token), opt_prev_token);
            }

            OperKind::CloseParen => {
                // Find matching open parenthesis.
                let mut found_open_paren = false;
                while let Some(ref_token) = self.stack_op.last() {
                    match ref_token {
                        Token::Oper(OperToken { idx_oper, .. })
                                if OPERS[*idx_oper].kind == OperKind::OpenParen => {
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
                    *opt_prev_token = Some(Token::Oper(oper_token));

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

            OperKind::ParamSep => {
                // Find the previous open parenthesis.
                while let Some(ref_token) = self.stack_op.last() {
                    match ref_token {
                        Token::Oper(OperToken { idx_oper, .. })
                            if OPERS[*idx_oper].kind == OperKind::OpenParen => break,
                        _ => self.pop_move_to_output_queue(),
                    }
                }

                // If a token exists at the top of the op stack, it's an open parenthesis (due to the loop above).
                // This is debug asserted below for paranoia.
                if self.stack_op.last().is_some() {
                    let paren_token = self.stack_op.pop().unwrap();
                    #[cfg(debug_assertions)]
                    {
                        let oper_paren = OperToken::try_from(paren_token).unwrap();
                        debug_assert!(OPERS[oper_paren.idx_oper].kind == OperKind::OpenParen);
                    }

                    // If a function preceeds the open parenthesis, increment its parameter count by 1
                    // and re-push the function and the previously popped open parenthesis back to the
                    // op stack. It is important we do -NOT- update "opt_prev_token" while doing this
                    // temporary modification of a token's data in stack.
                    if let Some(mut func_token) = self.pop_func_from_stack() {
                        func_token.params += 1;
                        self.stack_op.push(Token::Func(func_token));
                        self.stack_op.push(paren_token);
                    } else {
                        // No function preceeding open parenthesis for a parameter separator. E.g "(32,5)"
                        let message = format!("for parameter separator '{}' at {}", oper.name, oper_token.idx_expr);
                        trace!("{:?} {}", ExprErrorKind::MissingFunction, message);
                        return Err(ExprError { idx_expr: oper_token.idx_expr,
                                               kind: ExprErrorKind::MissingFunction,
                                               message });
                    }
                } else {
                    // No matching open parenthesis for the parameter separator. E.g "32,4".
                    let message = format!("for parameter separator '{}' at {}", oper.name, oper_token.idx_expr);
                    trace!("{:?} {}", ExprErrorKind::MissingParenthesis, message);
                    return Err(ExprError { idx_expr: oper_token.idx_expr,
                                           kind: ExprErrorKind::MissingParenthesis,
                                           message });
                }
            }

            _ => {
                // Validate left associative operator.
                // NOTE: We could squeeze this into parse_operator() but this gives us better error messages
                // in some cases (see integration test).
                if oper.assoc == OperAssoc::Left {
                    // Assume we've parsed left-associative operator "<<".
                    // Rules for previous token are:
                    //   - Must exist. E.g. "<< 2" is invalid but we've already handled this in parse_oeprator.
                    //     We simply debug asserted it below for parnoia.
                    //   - Must not be an operator (but close parenthesis is allowed)
                    //     E.g. "/ << 2" and "( << 2" are invalid but ") << 2" can be valid.
                    //   - Must not be a right associative operator.
                    debug_assert!(opt_prev_token.is_some());
                    match opt_prev_token {
                        Some(Token::Oper(
                            OperToken { idx_oper, .. })) if OPERS[*idx_oper].kind != OperKind::CloseParen => {
                            let message = format!("for operator '{}' at {}", oper.name, oper_token.idx_expr);
                            trace!("{:?} {}", ExprErrorKind::MissingOperand, message);
                            return Err(ExprError { idx_expr: oper_token.idx_expr,
                                                   kind: ExprErrorKind::MissingOperand,
                                                   message });
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
                            }

                            // Pop operator with higher priority (depending on associativity) to the output queue.
                            if token_stack_oper.prec < oper.prec
                                || (oper.assoc == OperAssoc::Left && oper.prec == token_stack_oper.prec) {
                                self.pop_move_to_output_queue();
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }

                self.push_to_op_stack(Token::Oper(oper_token), opt_prev_token);
            }
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
        if str_expr.starts_with(func.name) {
            idx_found = idx;
            is_found = true;
            break;
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
            return (Some(Number { integer: 0u64, float: 0f64 }), 1);
        }
    }

    // Rust string to number conversion FUNCS do not grok prefixes (e.g., "0xf" will have
    // to be fed to it as just "f"). So we aggregate the number into 'str_num' and keep track
    // of the length of any prefix that's already part of the expression in 'len_prefix' (as
    // done above). This also has a side effect in making the loop below faster as we eliminate
    // checks that doesn't need to happen on every iteration.
    let mut str_num = String::with_capacity(64);
    let mut has_dec_pt = false;
    let mut is_fp_exp_notation = false;

    debug_assert!(radix != 0);
    for chr in iter_expr {
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

    if str_num.is_empty() {
        // The number is "0" followed by some non-numeric character, return 0.
        if len_prefix == 1 {
            return (Some(Number { integer: 0u64, float: 0f64 }), 1);
        }
        // No numeric characters with or without a prefix, either way it's invalid.
        // E.g "0x", "0n" or "/".
        return (None, 0);
    } else if str_num.ends_with('.') {
        // Number ends in a decimal point, return invalid.
        return (None, 0);
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

fn parse_oper(str_expr: &str, opers: &[Oper], opt_prev_token: &mut Option<Token>) -> Option<usize> {
    debug_assert_eq!(str_expr.trim_start_matches(char::is_whitespace), str_expr);

    let mut is_found = false;
    let mut idx_found = 0;

    for (idx, op) in opers.iter().enumerate() {
        // If this is the first occurrence of this operator, record where we found it.
        // Otherwise, record the currently found operator only if its length exceeds that
        // of a previously found (i.e., we should be able to find "<<" and not stop at "<"),
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
                            || opers[*idx_oper].kind == OperKind::OpenParen {
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
            // NOTE: I've got rid of post/pre inc/dec. operators but this does handle the case
            // if I add it back.  Maybe error messages might not be great.
            else if op.assoc == OperAssoc::Right {
                match opt_prev_token {
                    None => (),
                    Some(Token::Oper(OperToken { idx_oper, .. })) => {
                        if opers[*idx_oper].assoc == OperAssoc::Right {
                            continue;
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
        trace!("found '{}' - {}", opers[idx_found].name, opers[idx_found].help);
        Some(idx_found)
    } else {
        None
    }
}

fn verify_prev_token_not_function(opt_prev_token: &Option<Token>) -> Result<(), ExprError> {
    match opt_prev_token {
        Some(Token::Func(FuncToken { idx_func, idx_expr, .. })) => {
            let idx_open_paren = idx_expr + FUNCS[*idx_func].name.len();
            let message = format!("at {} for function '{}'", idx_open_paren, &FUNCS[*idx_func].name);
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
        Some(Token::Oper(
            OperToken { idx_oper, idx_expr })) if OPERS[*idx_oper].kind == OperKind::CloseParen => {
            let idx_oper_or_func = idx_expr + OPERS[*idx_oper].name.len();
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
            verify_prev_token_not_function(&opt_prev_token)?;
            verify_prev_token_not_close_paren(&opt_prev_token)?;
            trace!("number  : {} (0x{:x})", number.integer, number.integer);
            len_token = len_str;
            let num_token = NumToken { number, idx_expr: idx };
            expr_ctx.push_to_output_queue(Token::Num(num_token), &mut opt_prev_token);
        } else if let Some(idx_oper) = parse_oper(str_subexpr, &OPERS, &mut opt_prev_token) {
            debug_assert!(idx_oper < OPERS.len());
            // If the previous token was a function, this must be an open parenthesis.
            // E.g "avg +"; otherwise this is an invalid expression.
            if let Some(Token::Func(FuncToken { idx_func,
                                                idx_expr: idx_expr_func, .. })) = opt_prev_token {
                // Calculate where the open parenthesis must appear, we don't use "idx" because
                // it includes all the whitespace after the function name. We want to report the
                // character immediately after the name of the function.
                // E.g we want position X in "avgX Y+" rather than position Y.
                let idx_open_paren = idx_expr_func + FUNCS[idx_func].name.len();
                if OPERS[idx_oper].kind != OperKind::OpenParen {
                    let message = format!("at {} for function '{}'", idx_open_paren, &FUNCS[idx_func].name);
                    trace!("{:?} {}", ExprErrorKind::MissingParenthesis, message);
                    return Err(ExprError { idx_expr: idx_open_paren,
                                           kind: ExprErrorKind::MissingParenthesis,
                                           message });
                }
            }
            trace!("operator: {}", &OPERS[idx_oper].name);
            len_token = OPERS[idx_oper].name.len();
            let oper_token = OperToken { idx_oper, idx_expr: idx };
            expr_ctx.process_parsed_oper(oper_token, &mut opt_prev_token)?;
        } else if let Some(idx_func) = parse_function(str_subexpr, &FUNCS) {
            debug_assert!(idx_func < FUNCS.len());
            // If the previous token was a function, we have an invalid expression.
            // E.g "avg avg"; FUNCS must be followed by open parenthesis only.
            verify_prev_token_not_function(&opt_prev_token)?;
            trace!("function: {}", &FUNCS[idx_func].name);
            len_token = FUNCS[idx_func].name.len();
            let func_token = FuncToken { idx_func, idx_expr: idx, params: 0 };
            expr_ctx.push_to_op_stack(Token::Func(func_token), &mut opt_prev_token);
        } else {
            let message = format!("at {}", idx);
            trace!("{:?} {}", ExprErrorKind::InvalidExpr, message);
            return Err(ExprError { idx_expr: idx,
                                    kind: ExprErrorKind::InvalidExpr,
                                    message });
        }
        if len_token >= 2 {
            iter_str.nth(len_token - 2);
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
                    return Err(ExprError { idx_expr,
                                           kind: ExprErrorKind::InvalidParamCount,
                                           message });
                }
            }

            Token::Func(FuncToken { idx_func, idx_expr, params }) => {
                debug_assert!(idx_func < FUNCS.len());
                let function = &FUNCS[idx_func];
                if let Some(parameters) = expr_ctx.collect_params(params as usize, &mut stack_output) {
                    debug_assert!(parameters.len() == params as usize);
                    let res_expr = (function.evalfn)(idx_expr, &parameters)?;
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
        Err(ExprError { idx_expr: 0, kind: ExprErrorKind::InvalidExpr, message })
    }
}

#[cfg(test)]
mod unit_tests;

