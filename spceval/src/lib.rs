mod evaluator;
mod functions;
mod operators;

pub use evaluator::{ExprError, ExprErrorKind, Number, evaluate, max_sub_expressions};
