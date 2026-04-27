use sal_core::error::Error;
use sal_sync::sync::RwLock;
use std::{fmt::Debug, sync::Arc};

use crate::domain::{RawContext, Snapshot};

///
/// Transaction for the [Context]
pub struct ContextTransaction {
    // Владеем ссылкой на хранилище
    pub(super) origin: Arc<RwLock<Arc<RawContext>>>,
    // Локальная копия для внесения изменений
    pub(super) state: RawContext,
    // Снимок данных для DB и UI
    pub snapshot: Snapshot,
}
impl ContextTransaction {
    /// Complete transaction, apply all changes to the [Context]
    /// - Only if the original context wasn't changed
    pub fn commit(mut self) -> Result<(), (Self, Error)> {
        let mut origin_lock = self.origin.write();
        let origin_version = origin_lock.version;
        if origin_version == self.state.version {
            self.state.version += 1;
            *origin_lock = Arc::new(self.state);
            Ok(())
        } else {
            drop(origin_lock);
            let err = Error::new("ContextTransaction", "commit")
                .err(format!("Context already was changed, origin ver {}, but staged was {}", origin_version, self.state.version));
            Err((self, err))
        }
    }
    /// Complete transaction, apply all changes to the [Context]
    /// - Even if the original context was changed
    pub fn force_commit(mut self) -> Result<(), Error> {
        let mut origin_lock = self.origin.write();
        self.state.version = origin_lock.version + 1;
        *origin_lock = Arc::new(self.state);
        Ok(())
    }
    /// Cancel transaction, drop all changes
    pub fn rollback(self) {
        drop(self)
    }
}
impl Debug for ContextTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextTransaction").field("origin", &(*self.origin.read())).field("state", &self.state).finish()
    }
}
