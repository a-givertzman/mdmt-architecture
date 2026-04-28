use std::sync::Arc;

use arc_swap::ArcSwap;
use sal_core::dbg::Dbg;

use crate::{domain::{ProjectNode, ProjectNodeStatus}, kernel::types::fx_map::FxDashMap};

///
/// ### Коллекция узлов дерева проекта
pub struct ProjectNodes {
    nodes: FxDashMap<String, Arc<ProjectNode>>,
    updated_nodes: ArcSwap<FxDashMap<String, Arc<ProjectNode>>>,
    dbg: Dbg,
}
//
impl ProjectNodes {
    pub fn new(parent: impl Into<String>) -> Self {
        Self {
            nodes: FxDashMap::default(),
            updated_nodes: ArcSwap::new(Arc::new(FxDashMap::default())),
            dbg: Dbg::new(parent, "ProjectNodes"),
        }
    }
    ///
    /// ### Агрегация статусов дерева наверх
    /// 
    /// **Механизм агрегации**
    /// - Происходит по события в PT-link, это очередь событий
    /// - В каждом цикле (64...120мс) вычитываться полностью.
    /// - Пересчет делаем по всем полученным событиям
    /// - Затем отправка статусов в UI
    pub fn update_status(&self, node_id: usize, node_status: ProjectNodeStatus) {
        
    }
    ///
    /// Возвращает изменившиеся ноды
    /// - Если изменился статус
    pub fn get_updated(&self) -> Vec<(String, Arc<ProjectNode>)> {
        // Атомарно забираем все изменившиеся ноды и оставляем на их месте пустую мапу
        let updated = self.updated_nodes.swap(Arc::new(FxDashMap::default()));
        // Вытаскиваем ноды по ключам
        match Arc::try_unwrap(updated) {
            Ok(map) => map.into_iter().collect(),
            Err(arc_map) => {
                // Если кто-то еще держал ссылку на эту мапу, клонируем элементы
                arc_map.iter().map(|r| (r.key().clone(), r.value().clone())).collect()
            }
        }
    }
}

