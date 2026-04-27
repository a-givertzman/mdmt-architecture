mod context_transaction;
mod context;
mod initial_ctx;
mod initial;
mod raw_context;

pub use context_transaction::*;
pub use context::*;
pub use initial_ctx::*;
pub use initial::*;
pub use raw_context::*;

use sal_core::error::Error;

///
/// Provides IEC key of the Context members
pub trait IecId {
    fn iec_id() -> &'static str;
}
/// Provides restricted write access to the [ContextTransaction]
pub trait ContextWrite<T> where Self: Sized {
    fn write(self, value: T) -> Result<Self, Error>;
}
/// Provides simple read access to the [ContextTransaction] members
pub trait ContextReadRef<T> {
    fn read_ref(&self) -> &T;
}
/// Provides simple read access to the [ContextTransaction] members
pub trait ContextRead<T> {
    fn read(&self) -> T;
}
