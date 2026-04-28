use crate::domain::{ProjectNodeKind, ProjectNodeStatus};

///
/// ### Узел дерева проекта
pub struct ProjectNode {
    ///  - Идентификатор уникальный в пределах проекта (db.project_nodes.id)
    id: usize,
    /// - Принадлежность проекту (db.project_nodes.project_id)
    project_id: usize,
    ///  - связь с родителем (db.project_nodes.parent_id)
    parent_id: usize,
    /// - для сортировки в пределах родителя (db.project_nodes.node_order)
    order: usize,
    /// - Вид узла по ISO 10303 (db.project_nodes.kind)
    kind: ProjectNodeKind,
    /// - для [OCC](./reference/occ.md)
    version: usize,
    /// - Актуальное состояние
    status: ProjectNodeStatus,
    /// Связь с элементом 3D модели - и есть Классификация
    geometry_id: usize,
}
