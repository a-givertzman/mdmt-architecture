use sal_core::error::Error;
use crate::context::ContextTransaction;
///
/// Result returned from Calculation steps
pub type EvalResult = Result<ContextTransaction, Error>;