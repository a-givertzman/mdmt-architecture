use std::sync::Arc;

use crate::{context::Context, kernel::{Eval, types::eval_result::EvalResult}, snapshot::ApiClient};

use sal_core::{dbg::Dbg, error::Error};
use sal_sync::sync::channel;

///
/// Общая структура для ввода данных. Содержит все данные
/// для расчетов.
pub struct Initial {
    dbg: Dbg,
    ctx: Arc<Context>,
}
//
impl Initial {
    ///
    /// Fetches all initiall data
    /// - 'api_client' - access to the database
    pub fn new(parent: impl Into<String>, ctx: Arc<Context>) -> Self {
        let dbg = Dbg::new(parent, "Initial");
        Self {
            dbg,
            ctx,
        }
    }
}
//
impl Eval<(), EvalResult> for Initial {
    fn eval(&self, _: ()) -> EvalResult {
        let (link, _) = channel::unbounded();
        let client = Arc::new(ApiClient {});
        Ok(self.ctx.transaction(link, client))
    }
}
