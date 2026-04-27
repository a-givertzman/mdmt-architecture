use sal_core::dbg::Dbg;

use crate::{domain::ProjectNode, kernel::types::fx_map::FxDashMap};

///
/// ### Коллекция узлов дерева проекта
pub struct ProjectNodes {
    nodes: FxDashMap<String, ProjectNode>,
    dbg: Dbg,
}
//
impl ProjectNodes {
    pub fn new(parent: impl Into<String>) -> Self {
        Self {
            nodes: FxDashMap::default(),
            dbg: Dbg::new(parent, "ProjectNodes"),
        }
    }
}

