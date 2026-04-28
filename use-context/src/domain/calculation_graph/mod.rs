///
/// Calculation Dependency node contains IecKey's of the members accessed on the context for the single calculation step
#[derive(Debug)]
pub struct CalculationTags {
    /// members read from the context
    pub read: Vec<&'static str>,
    /// members stored into the context
    pub write: Vec<&'static str>,
}
///
/// Calculation-Graph
pub trait EvalTags {
    fn tags() -> CalculationTags;
    // fn tags(&self) -> CalculationTags;
}