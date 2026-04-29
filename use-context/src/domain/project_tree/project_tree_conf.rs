use serde::Deserialize;
use std::time::Duration;

///
/// ## Config for `ProjectTree` format:
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ProjectTreeConf {
    /// Next service will wait until current completely started plus specified time, optional
    #[serde(rename="wait-started")]
    pub wait_started: Option<Duration>,
}
