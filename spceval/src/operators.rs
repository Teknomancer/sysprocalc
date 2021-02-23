use super::*;

pub static OPERS: [Oper<'static>; 25] = [
    // Precedence 1 (highest priority)
    Oper { kind: OperKind::OpenParen,  prec: 1,  params: 0, assoc: OperAssoc::Nil,   evalfn: oper_nop,         name: "(",  syntax: "(<expr>",            help: "Begin expression.",       },
    Oper { kind: OperKind::CloseParen, prec: 1,  params: 0, assoc: OperAssoc::Nil,   evalfn: oper_nop,         name: ")",  syntax: "<expr>)",            help: "End expression.",         },
    // Precendence 4 (appears in array before 2 because of parsing logic with unary operators)
    Oper { kind: OperKind::Regular,    prec: 4,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_add,         name: "+",  syntax: "<expr> + <expr>",    help: "Addition.",               },
    Oper { kind: OperKind::Regular,    prec: 4,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_sub,         name: "-",  syntax: "<expr> - <expr>",    help: "Subtraction.",            },
    // Precedence 2
    Oper { kind: OperKind::Regular,    prec: 2,  params: 1, assoc: OperAssoc::Right, evalfn: oper_nop,         name: "+",  syntax: "+<expr>",            help: "Unary plus.",             },
    Oper { kind: OperKind::Regular,    prec: 2,  params: 1, assoc: OperAssoc::Right, evalfn: oper_unary_minus, name: "-",  syntax: "-<expr>",            help: "Unary minus.",            },
    Oper { kind: OperKind::Regular,    prec: 2,  params: 1, assoc: OperAssoc::Right, evalfn: oper_logical_not, name: "!",  syntax: "!<expr>",            help: "Logical NOT.",            },
    Oper { kind: OperKind::Regular,    prec: 2,  params: 1, assoc: OperAssoc::Right, evalfn: oper_bit_not,     name: "~",  syntax: "~<expr>",            help: "Bitwise NOT.",            },
    // Precedence 3
    Oper { kind: OperKind::Regular,    prec: 3,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_mul,         name: "*",  syntax: "<expr> * <expr>",    help: "Multiplication.",         },
    Oper { kind: OperKind::Regular,    prec: 3,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_div,         name: "/",  syntax: "<expr> / <expr>",    help: "Division.",               },
    Oper { kind: OperKind::Regular,    prec: 3,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_rem,         name: "%",  syntax: "<expr> % <expr>",    help: "Remainder.",              },
    // Precedence 5
    Oper { kind: OperKind::Regular,    prec: 5,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_bit_lshift,  name: "<<", syntax: "<expr> << <expr>",   help: "Bitwise left-shift.",     },
    Oper { kind: OperKind::Regular,    prec: 5,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_bit_rshift,  name: ">>", syntax: "<expr> >> <expr>",   help: "Bitwise right-shift.",    },
    // Precedence 6
    Oper { kind: OperKind::Regular,    prec: 6,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_lt,          name: "<",  syntax: "<expr> < <expr>",    help: "Less-than.",              },
    Oper { kind: OperKind::Regular,    prec: 6,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_lte,         name: "<=", syntax: "<expr> <= <expr>",   help: "Less-than-or-equals.",    },
    Oper { kind: OperKind::Regular,    prec: 6,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_gt,          name: ">",  syntax: "<expr> > <expr>",    help: "Greater-than.",           },
    Oper { kind: OperKind::Regular,    prec: 6,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_gte,         name: ">=", syntax: "<expr> >= <expr>",   help: "Greater-than-or-equals.", },
    // Precedence 7
    Oper { kind: OperKind::Regular,    prec: 7,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_eq,          name: "==", syntax: "<expr> == <expr>",   help: "Equals.",                 },
    Oper { kind: OperKind::Regular,    prec: 7,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_ne,          name: "!=", syntax: "<expr> != <expr>",   help: "Not-equals.",             },
    // Precedence 8
    Oper { kind: OperKind::Regular,    prec: 8,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_bit_and,     name: "&",  syntax: "<expr> & <expr>",    help: "Bitwise AND.",            },
    // Precedence 9
    Oper { kind: OperKind::Regular,    prec: 9,  params: 2, assoc: OperAssoc::Left,  evalfn: oper_bit_xor,     name: "^",  syntax: "<expr> ^ <expr>",    help: "Bitwise XOR.",            },
    // Precedence 10
    Oper { kind: OperKind::Regular,    prec: 10, params: 2, assoc: OperAssoc::Left,  evalfn: oper_bit_or,      name: "|",  syntax: "<expr> | <expr>",    help: "Bitwise OR." ,            },
    // Precedence 11
    Oper { kind: OperKind::Regular,    prec: 11, params: 2, assoc: OperAssoc::Left,  evalfn: oper_nop,         name: "&&", syntax: "<expr> && <expr>",   help: "Logical AND.",            },
    // Precedence 12
    Oper { kind: OperKind::Regular,    prec: 12, params: 2, assoc: OperAssoc::Left,  evalfn: oper_nop,         name: "||", syntax: "<expr> || <expr>",   help: "Logical OR." ,            },
    // Precedence 13
    Oper { kind: OperKind::ParamSep,   prec: 13, params: 2, assoc: OperAssoc::Left,  evalfn: oper_nop,         name: ",",  syntax: "<param1>, <param2>", help: "Parameter separator.",    },
];

type PfnOper = fn(idx_expr: usize, &[Number]) -> Result<Number, ExprError>;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum OperAssoc {
    Nil,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum OperKind {
    Regular,
    OpenParen,
    CloseParen,
    ParamSep,
}

pub struct Oper<'a> {
    pub kind: OperKind,
    pub prec: u8,
    pub params: u8,
    pub assoc: OperAssoc,
    pub evalfn: PfnOper,
    pub name: &'a str,
    pub syntax: &'a str,
    pub help: &'a str,
}

// Eq specifies that the equality relationship defined by PartialEq is a total equality.
impl Eq for Oper<'_> {}

// PartialEq is required by PartialOrd which is required for sorting.
impl PartialEq for Oper<'_> {
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
impl Ord for Oper<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.name.cmp(&self.name)
    }
}

// Ord specifies that the ordering relationship defined by PartialOrd is total ordering.
impl PartialOrd for Oper<'_> {
    fn partial_cmp(&self, other: &Oper) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cmp_eq_f64(a: f64, b: f64) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let abs_diff = (a - b).abs();
    let abs_cmp = if abs_a > abs_b { abs_b } else { abs_a };

    abs_diff <= abs_cmp * std::f64::EPSILON
}

fn oper_nop(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    Ok (nums[0])
}

fn oper_add(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.wrapping_add(rhs.integer);
    let float = lhs.float + rhs.float;
    Ok(Number { integer, float })
}

fn oper_sub(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.wrapping_sub(rhs.integer);
    let float = lhs.float - rhs.float;
    Ok(Number { integer, float })
}

fn oper_unary_minus(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let rhs = nums[0];
    let integer = rhs.integer.wrapping_neg();
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
    let integer = lhs.integer.wrapping_mul(rhs.integer);
    let float = lhs.float * rhs.float;
    Ok(Number { integer, float })
}

fn oper_div(idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    if  rhs.integer != 0 && !cmp_eq_f64(rhs.float, 0f64) {
        let integer = lhs.integer.wrapping_div(rhs.integer);
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
    let integer = lhs.integer.wrapping_rem(rhs.integer);
    let float = lhs.float % rhs.float;
    Ok(Number { integer, float })
}

fn oper_bit_lshift(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.wrapping_shl(rhs.integer as u32);
    let float = integer as f64;
    Ok(Number { integer, float })
}

fn oper_bit_rshift(_idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let lhs = nums[0];
    let rhs = nums[1];
    let integer = lhs.integer.wrapping_shr(rhs.integer as u32);
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

