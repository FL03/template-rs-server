/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::types::Uid;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct WorkerConfig {
    #[serde(default = "rand::random")]
    pub id: Uid,
    #[serde(default)]
    pub(crate) name: String,
}

impl WorkerConfig {
    pub fn new() -> Self {
        Self {
            id: rand::random(),
            name: String::new(),
        }
    }

    pub fn id(&self) -> Uid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl ToString) {
        self.name = name.to_string();
    }

    pub fn with_id(self, id: Uid) -> Self {
        Self { id, ..self }
    }

    pub fn with_name(self, name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            ..self
        }
    }
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self::new()
    }
}
