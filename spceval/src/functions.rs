use crate::{Number, ExprError, ExprErrorKind};
use std::ops::Range;

const KB_IN_B: u64 = 0x400;
const MB_IN_B: u64 = 0x100000;
const GB_IN_B: u64 = 0x40000000;
const TB_IN_B: u64 = 0x10000000000;
const PB_IN_B: u64 = 4000000000000;

pub const MAX_FN_PARAMS: u8 = u8::max_value();
pub static FUNCS: [Func<'static>; 15] = [
    Func {
        name:   "avg",
        params: Range { start: 2, end: MAX_FN_PARAMS },
        syntax: "<n1>,<n2>[,<n3>...<nX>]",
        help:   "Average",
        evalfn: func_avg,
    },
    Func {
        name:   "b2gb",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Bytes to gigabytes",
        evalfn: func_b2gb,
    },
    Func {
        name:   "b2kb",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Bytes to kilobytes",
        evalfn: func_b2kb,
    },
    Func {
        name:   "b2mb",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Bytes to megabytes",
        evalfn: func_b2mb,
    },
    Func {
        name:   "b2pb",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Bytes to petabytes",
        evalfn: func_b2pb,
    },
    Func {
        name:   "b2tb",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Bytes to terabytes",
        evalfn: func_b2tb,
    },
    Func {
        name:   "bit",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Set nth bit (n is [0..63])",
        evalfn: func_bit,
    },
    Func {
        name:   "bits",
        params: Range { start: 2, end: 3 },
        syntax: "<n1>,<n2>",
        help:   "Set set of bits from [n1..n2]",
        evalfn: func_bits,
    },
    Func {
        name:   "gb2b",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Gigabytes to bytes",
        evalfn: func_gb2b,
    },
    Func {
        name:   "is_pow_of_two",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Is power of 2",
        evalfn: func_is_pow_of_two,
    },
    Func {
        name:   "kb2b",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Kilobytes to bytes",
        evalfn: func_kb2b,
    },
    Func {
        name:   "mb2b",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Megabytes to bytes",
        evalfn: func_mb2b,
    },
    Func {
        name:   "pb2b",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Petabytes to bytes",
        evalfn: func_pb2b,
    },
    Func {
        name:   "sum",
        params: Range { start: 2, end: MAX_FN_PARAMS },
        syntax: "<n1>,<n2>[,<n3>..<nX>]",
        help:   "Sum",
        evalfn: func_sum,
    },
    Func {
        name:   "tb2b",
        params: Range { start: 1, end: 2 },
        syntax: "<n>",
        help:   "Terabytes to bytes",
        evalfn: func_tb2b,
    },
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

fn func_b2kb(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let integer = nums[0].integer / KB_IN_B;
    let float = nums[0].float / KB_IN_B as f64;
    Ok(Number { integer, float })
}

fn func_kb2b(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let integer = nums[0].integer * KB_IN_B;
    let float = nums[0].float * KB_IN_B as f64;
    Ok(Number { integer, float })
}

fn func_b2mb(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let integer = nums[0].integer / MB_IN_B;
    let float = nums[0].float / MB_IN_B as f64;
    Ok(Number { integer, float })
}

fn func_mb2b(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let integer = nums[0].integer * MB_IN_B;
    let float = nums[0].float * MB_IN_B as f64;
    Ok(Number { integer, float })
}

fn func_b2gb(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let integer = nums[0].integer / GB_IN_B;
    let float = nums[0].float / GB_IN_B as f64;
    Ok(Number { integer, float })
}

fn func_gb2b(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let integer = nums[0].integer * GB_IN_B;
    let float = nums[0].float * GB_IN_B as f64;
    Ok(Number { integer, float })
}

fn func_b2tb(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let integer = nums[0].integer / TB_IN_B;
    let float = nums[0].float / TB_IN_B as f64;
    Ok(Number { integer, float })
}

fn func_tb2b(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let integer = nums[0].integer * TB_IN_B;
    let float = nums[0].float * TB_IN_B as f64;
    Ok(Number { integer, float })
}

fn func_b2pb(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let integer = nums[0].integer / PB_IN_B;
    let float = nums[0].float / PB_IN_B as f64;
    Ok(Number { integer, float })
}

fn func_pb2b(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let integer = nums[0].integer * PB_IN_B;
    let float = nums[0].float * PB_IN_B as f64;
    Ok(Number { integer, float })
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
        Err(ExprError::new(idx_expr, ExprErrorKind::FailedEvaluation, message))
    }
}

fn func_bits(func: &Func, idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let min = std::cmp::min(nums[0].integer, nums[1].integer) as u32;
    let max = std::cmp::max(nums[0].integer, nums[1].integer) as u32;
    if (0..u64::BITS).contains(&min) && (0..u64::BITS).contains(&max) {
        let mut integer : u64 = 0;
        for n in min..max + 1 {
            integer |= 1_u64.wrapping_shl(n);
        }
        let float = integer as f64;
        Ok(Number { integer, float })
    } else {
        let message = format!("for function '{}' at {} due to invalid bit range ({}, {}) (must be 0..63)",
                              func.name, idx_expr, nums[0].integer as i64, nums[1].integer as i64);
        Err(ExprError::new(idx_expr, ExprErrorKind::FailedEvaluation, message))
    }
}

fn func_is_pow_of_two(_func: &Func, _idx_expr: usize, nums: &[Number]) -> Result<Number, ExprError> {
    let val = nums[0].integer as u64;
    let integer = if val > 0 {
        (val & (val - 1) == 0) as u64
    } else {
        0
    };
    let float = integer as f64;
    Ok (Number { integer, float })
}

