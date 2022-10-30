mod functions;
mod operators;
mod evaluator;

pub use evaluator::{ ExprError, Number, ExprErrorKind, evaluate, max_sub_expressions };

