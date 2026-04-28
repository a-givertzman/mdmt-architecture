use crate::domain::{ProjectNodeKind, ProjectNodeStatus};

///
/// ### Узел дерева проекта
#[derive(Debug, Clone, Copy)]
pub struct ProjectNode {
    /// - Принадлежность проекту (db.project_nodes.project_id)
    pub project_id: usize,
    ///  - Идентификатор уникальный в пределах проекта (db.project_nodes.id)
    pub id: usize,
    ///  - связь с родителем (db.project_nodes.parent_id)
    pub parent_id: usize,
    /// - для сортировки в пределах родителя (db.project_nodes.node_order)
    pub order: usize,
    /// - Вид узла по ISO 10303 (db.project_nodes.kind)
    pub kind: ProjectNodeKind,
    /// - для [OCC](./reference/occ.md)
    version: usize,
    /// - Актуальное состояние
    pub status: ProjectNodeStatus,
    /// Связь с элементом 3D модели - и есть Классификация
    pub geometry_id: usize,
}
//
impl ProjectNode {
    ///
    /// Returns new [ProjectNode] instance with default `status` and `version = 0`
    /// - `project_id`
    /// - `id` - Идентификатор уникальный в пределах проекта (db.project_nodes.id)
    /// - `parent_id`
    /// - `order`
    /// - `kind`
    /// - `geometry_id`
    pub fn new(project_id: usize, id: usize, parent_id: usize, order: usize, kind: ProjectNodeKind, geometry_id: usize) -> Self {
        Self {
            project_id,
            id,
            parent_id,
            order,
            kind,
            version: 0,
            status: ProjectNodeStatus::default(),
            geometry_id,
        }
    }
}