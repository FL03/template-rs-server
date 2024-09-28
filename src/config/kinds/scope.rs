/*
    Appellation: scope <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::{path::PathBuf, str::FromStr};

fn default_ctx() -> Option<String> {
    Some(format!("/etc/pzzld"))
}

fn default_workdir() -> String {
    format!("dist")
}

/// [Scope] is a structure containing all of the information required for the service to operate.
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Scope {
    #[serde(default = "default_ctx")]
    // The root directory of the service
    pub(crate) context: Option<String>,
    // The directory where all of the assets
    #[serde(default = "default_workdir")]
    pub(crate) workdir: String,
}

impl Scope {
    pub fn new(workdir: impl ToString) -> Self {
        debug_assert!(PathBuf::from_str(workdir.to_string().as_str()).is_ok());
        Self {
            context: None,
            workdir: workdir.to_string(),
        }
    }

    pub fn as_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        if let Some(context) = &self.context {
            path.push(context);
        }
        path.push(&self.workdir);
        path
    }

    pub fn context(&self) -> &str {
        self.context.as_deref().unwrap_or(".")
    }

    pub fn display(&self) -> String {
        self.as_path().display().to_string()
    }

    pub fn workdir(&self) -> &str {
        &self.workdir
    }

    pub fn set_context(&mut self, context: impl ToString) {
        self.context = Some(context.to_string());
    }

    pub fn set_workdir(&mut self, workdir: impl ToString) {
        self.workdir = workdir.to_string();
    }

    pub fn set_some_workdir(&mut self, workdir: Option<impl ToString>) {
        if let Some(workdir) = workdir {
            self.workdir = workdir.to_string();
        }
    }

    pub fn with_context(self, context: impl ToString) -> Self {
        Self {
            context: Some(context.to_string()),
            ..self
        }
    }

    pub fn with_workdir(self, workdir: impl ToString) -> Self {
        Self {
            workdir: workdir.to_string(),
            ..self
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            context: None,
            workdir: "dist".into(),
        }
    }
}

impl core::fmt::Display for Scope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{path}", path = self.as_path().display())
    }
}

impl core::str::FromStr for Scope {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
