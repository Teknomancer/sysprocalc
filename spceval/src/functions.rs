use super::{Number, ExprError, ExprErrorKind};
use std::ops::Range;

pub const MAX_FN_PARAMS: u8 = u8::max_value();
pub static FUNCS: [Func<'static>; 4] = [
    Func { name: "avg",
           params: Range { start: 2, end: MAX_FN_PARAMS },
           syntax: "<n1>,<n2>[,<n3>...<nX>]",
           help: "Average",
           evalfn: func_avg, },
    Func { name: "bit", params: Range { start: 1, end: 2 },
           syntax: "<n>",
           help: "Set nth bit (n is [0..63])",
           evalfn: func_bit, },
    Func { name: "bits",
           params: Range { start: 2, end: 3 },
           syntax: "<n1>,<n2>",
           help: "Set set of bits from [n1..n2]",
           evalfn: func_bits, },
    Func { name: "sum",
           params: Range { start: 2, end: MAX_FN_PARAMS },
           syntax: "<n1>,<n2>[,<n3>..<nX>]",
           help: "Sum",
           evalfn: func_sum, },
];

type PfnFunc = fn(func: &Func, idx_expr: usize, &[Number]) -> Result<Number, ExprError>;

pub struct Func<'a> {
    pub name: &'a str,
    pub params: Range<u8>,
    pub syntax: &'a str,
    pub help: &'a str,
    pub evalfn: PfnFunc,
}

fn func_sum__(nums: &[Number]) -> Result<Number, ExprError>
{
    let mut res = Number { integer: 0u64, float: 0f64 };
    for arg in nums  {
        res.integer = res.integer.wrapping_add(arg.integer);
        res.float += arg.float;
    }
    Ok(res)
}

fn func_sum(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    func_sum__(nums)
}

fn func_avg(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let mut res = func_sum__(nums)?;
    res.integer /= nums.len() as u64;
    res.float /= nums.len() as f64;
    Ok(res)
}

fn func_bit(func: &Func, idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let shift = nums[0].integer as u32;
    if (0..u64::BITS).contains(&shift) {
        let integer = 1_u64.wrapping_shl(nums[0].integer as u32);
        let float = integer as f64;
        Ok(Number { integer, float })
    } else {
        let message = format!("for function '{}' at {} due to invalid shift {} (must be 0..63)",
                              func.name, idx_expr, nums[0].integer as i64);
        Err(ExprError { idx_expr, kind: ExprErrorKind::FailedEvaluation, message })
    }
}

fn func_bits(func: &Func, idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let min = std::cmp::min(nums[0].integer, nums[1].integer) as u32;
    let max = std::cmp::max(nums[0].integer, nums[1].integer) as u32;
    if (0..u64::BITS).contains(&min) && (0..u64::BITS).contains(&max) {
        let mut integer : u64 = 0;
        for n in min..max + 1  {
            integer |= 1_u64.wrapping_shl(n as u32);
        }
        let float = integer as f64;
        Ok(Number { integer, float })
    } else {
        let message = format!("for function '{}' at {} due to invalid bit range ({}, {}) (must be 0..63)",
                              func.name, idx_expr, nums[0].integer as i64, nums[1].integer as i64);
        Err(ExprError { idx_expr, kind: ExprErrorKind::FailedEvaluation, message })
    }
}

