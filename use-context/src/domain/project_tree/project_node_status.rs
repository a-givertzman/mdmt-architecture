///
/// ### Статусы узлов дерева проекта
/// 
/// | Приоритет | Иконка |       Имя        |                          Комментарий                               |
/// |    ---    |   ---  |       ---        |                              ---                                   |
/// |     1     |   ❌   | Error            | Блокирующая ошибка расчетов или системы. Перекрывает всё.          |
/// |     2     |   ⛔   | NoClassification | Модель импортирована, но не размечена. Расчеты невозможны.         |
/// |     3     |   ❓   | NoData           | Отсутствуют критически важные исходные данные.                     |
/// |     4     |   ⚡   | Inconsistency    | Неконсистентность данных.                                          |
/// |     5     |   ⏳   | Calculating      | Процесс идет, ждем результатов.                                    |
/// |     6     |   ⚠️   | Outdated         | Данные изменились, нужен пересчет, но старые результаты еще висят. |
/// |     7     |   ✔️   | Ready            | Всё идеально.                                                      |
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectNodeStatus {
    Error = 1,
    NoClassification = 2,
    NoData = 3,
    Inconsistency = 4,
    Calculating = 5,
    Outdated = 6,
    Ready = 7,
}
//
impl Default for ProjectNodeStatus {
    fn default() -> Self {
        Self::Outdated
    }
}