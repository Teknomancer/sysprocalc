mod functions;
mod operators;
pub mod evaluator;

pub use evaluator::{ ExprError, Number, ExprErrorKind, evaluate, max_sub_expressions };

