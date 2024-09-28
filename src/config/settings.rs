/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{LoggerConfig, NetworkConfig, Scope};
use crate::types::Mode;
use crate::workers::WorkerConfig;

use config::{
    builder::{ConfigBuilder, DefaultState},
    ConfigError,
};

fn set_default(
    builder: ConfigBuilder<DefaultState>,
) -> Result<ConfigBuilder<DefaultState>, ConfigError> {
    builder
        .set_default("mode", "debug")?
        .set_default("log.level", "info")?
        .set_default("scope.context", ".")?
        .set_default("scope.workdir", crate::DEFAULT_WORKDIR)?
        .set_default("server.name", crate::APP_NAME)?
        .set_default("server.port", crate::DEFAULT_PORT)
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct Settings {
    #[serde(default)]
    pub log: LoggerConfig,
    #[serde(default)]
    pub mode: Mode,
    #[serde(default)]
    pub scope: Scope,
    #[serde(default)]
    pub server: NetworkConfig,
    #[serde(default)]
    pub worker: Vec<WorkerConfig>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            log: LoggerConfig::default(),
            mode: Mode::Debug,
            scope: Scope::default(),
            server: NetworkConfig::default(),
            worker: Vec::new(),
        }
    }

    fn builder_base() -> Result<ConfigBuilder<DefaultState>, ConfigError> {
        let mut builder = config::Config::builder();

        builder = set_default(builder)?;

        // env vars
        builder = builder.add_source(config::Environment::with_prefix("PZZLD").separator("_"));
        // set overrides
        builder = builder
            .set_override_option("mode", std::env::var("MODE").ok())?
            .set_override_option("log.level", std::env::var("RUST_LOG").ok())?;

        builder = if let Ok(var) = std::env::var("PZZLD_SETTINGS") {
            builder.add_source(config::File::with_name(&var).required(false))
        } else {
            builder.add_source(config::File::with_name("Puzzled").required(false))
        };

        Ok(builder)
    }

    pub fn build() -> Result<Self, config::ConfigError> {
        Self::builder_base()?.build()?.try_deserialize()
    }
    /// Returns the current logger configuration.
    pub fn log(&self) -> LoggerConfig {
        self.log
    }
    /// Returns the current [Mode] of the application.
    pub fn mode(&self) -> Mode {
        self.mode
    }
    /// Returns an immutable reference to the scope configuration.
    pub const fn scope(&self) -> &Scope {
        &self.scope
    }
    /// Returns a mutable reference to the scope configuration.
    pub fn scope_mut(&mut self) -> &mut Scope {
        &mut self.scope
    }
    /// Returns an immutable reference to the network configuration.
    pub const fn network(&self) -> &NetworkConfig {
        &self.server
    }
    /// Returns a mutable reference to the network configuration.
    pub fn server_mut(&mut self) -> &mut NetworkConfig {
        &mut self.server
    }
    /// Returns an immutable reference to the worker configuration.
    pub fn workers(&self) -> &[WorkerConfig] {
        &self.worker
    }
    /// Initialize tracing modules
    pub fn init_tracing(&self) {
        self.log.init_tracing(crate::APP_NAME);
    }

    setwith! {
        log: LoggerConfig,
        mode: Mode,
        scope: Scope,
        server: NetworkConfig,
    }

    pub fn push_worker(&mut self, worker: WorkerConfig) {
        self.worker.push(worker)
    }

    pub fn set_workdir(&mut self, workdir: impl ToString) {
        self.scope.set_workdir(workdir);
    }

    pub fn set_some_workdir(&mut self, workdir: Option<impl ToString>) {
        self.scope.set_some_workdir(workdir);
    }

    pub fn set_scope_context(&mut self, context: impl ToString) {
        self.scope.set_context(context);
    }

    pub fn set_server_port(&mut self, port: u16) {
        self.server.set_port(port);
    }

    pub fn set_server_name(&mut self, name: impl ToString) {
        self.server.set_name(name);
    }

    pub fn set_log_level(&mut self, level: super::LogLevel) {
        self.log.set_level(level);
    }
}
