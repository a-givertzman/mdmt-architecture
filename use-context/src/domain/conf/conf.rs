use std::{fs::OpenOptions, path::Path};

use sal_core::error::Error;
use serde::Deserialize;

use crate::domain::ProjectTreeConf;

///
/// Application configuration
#[derive(Debug, Deserialize)]
pub struct Conf {
    #[serde(rename="thread-pool")]
    pub thread_pool: Option<usize>,
    #[serde(rename="project-tree")]
    pub project_tree: ProjectTreeConf,
}
//
impl Conf {
    pub fn read(path: impl AsRef<Path>) -> Result<Self, Error> {
        let error = Error::new("Conf", "new");
        let rdr = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|err| error.pass(err.to_string()))?;
        let conf = serde_yaml::from_reader(rdr)
            .map_err(|err| error.pass(err.to_string()))?;
        Ok(conf)
    }
}