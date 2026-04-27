use get_size::GetSize;
use context_macros::ContextAccess;
use std::fmt::Debug;

use crate::{algorithm::{ApparentFrequenciesCtx, Parameters, UnitAreaCtx}, domain::InitialCtx};

/// Сырой контекст для вычислений
/// Без изменений берем из SSS
#[derive(Debug, Clone, Default, ContextAccess, GetSize)]
pub struct RawContext {
    /// Контроль версий для консистентности
    #[context(skip)]
    pub(super) version: usize,
    #[context(read, read_ref)]
    pub(super) initial: InitialCtx,
    #[context(read, read_ref, write)]
    pub(super) apparent_frequencies: Option<ApparentFrequenciesCtx>,
    #[context(read, read_ref, write)]
    pub(super) parameters: Parameters,
    #[context(read, read_ref, write)]
    pub(super) unit_area: Option<UnitAreaCtx>,
    // ...
}
impl RawContext {
    ///
    /// New instance [RawContext]
    /// - 'initial' - [InitialCtx] instance, where store initial data
    pub fn new(initial: InitialCtx) -> Self {
        Self {
            initial,
            ..Self::default()
        }
    }
}
