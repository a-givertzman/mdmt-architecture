use get_size::GetSize;
use sal_context_macros::ContextProperties;
use serde::Serialize;

///
/// Результаты расчета массива [кажущихся частот волнения](https://github.com/a-givertzman/sss/blob/50-guidance-to-the-master-according-to-msc1-circ1228/design/algorithm/part06_seakeeping/part06_seakeeping.md#порядок-расчета)
#[derive(Debug, Clone, Serialize, ContextProperties, GetSize)]
#[iec_id = "Ship.Stability.ApparentFrequencies"]
pub struct ApparentFrequenciesCtx {
    /// Массив [кажущихся частот волнения](https://github.com/a-givertzman/sss/blob/50-guidance-to-the-master-according-to-msc1-circ1228/design/algorithm/part06_seakeeping/part06_seakeeping.md#порядок-расчета)
    pub apparent_frequencies: Vec<(f64, f64, f64)>, // (angle, speed, apparent_frequency)
}
