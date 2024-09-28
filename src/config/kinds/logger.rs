/*
    Appellation: log <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

fn default_true() -> bool {
    true
}

#[derive(
    Clone,
    Copy,
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
pub struct LoggerConfig {
    #[serde(default = "default_true")]
    pub(crate) ansi: bool,
    #[serde(default = "LogLevel::default")]
    pub(crate) level: LogLevel,
    #[serde(default = "default_true")]
    pub(crate) target: bool,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Error,
    Info,
    Trace,
    Warn,
    #[default]
    Off,
}

impl LoggerConfig {
    pub fn new(level: LogLevel) -> Self {
        Self {
            ansi: true,
            level,
            target: false,
        }
    }

    pub fn with_level(self, level: LogLevel) -> Self {
        Self { level, ..self }
    }

    pub fn level(&self) -> LogLevel {
        self.level
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }
    /// Initialize the tracer with the given name
    pub fn init_tracing(&self, name: &str) {
        use tracing_subscriber::{
            filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt,
        };

        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{name}={level},tower_http={level}", level = self.level).into()
        });

        let layer = tracing_subscriber::fmt::layer()
            .compact()
            .with_ansi(self.ansi)
            .with_target(self.target)
            .with_timer(tracing_subscriber::fmt::time::uptime());

        tracing_subscriber::registry()
            .with(filter)
            .with(layer)
            .init();
        tracing::info!("Success: initialized tracing");
    }
}
mod log_level {
    use super::LogLevel;

    impl LogLevel {
        pub fn from_str<S>(s: S) -> Self
        where
            S: AsRef<str>,
        {
            match s.as_ref() {
                "debug" => Self::Debug,
                "error" => Self::Error,
                "info" => Self::Info,
                "trace" => Self::Trace,
                "warn" => Self::Warn,
                _ => Self::Off,
            }
        }

        pub fn from_tracing(level: tracing::Level) -> Self {
            use tracing::Level;

            match level {
                Level::DEBUG => Self::Debug,
                Level::ERROR => Self::Error,
                Level::INFO => Self::Info,
                Level::TRACE => Self::Trace,
                Level::WARN => Self::Warn,
            }
        }

        pub fn debug() -> Self {
            Self::Debug
        }

        pub fn info() -> Self {
            Self::Info
        }

        pub fn warn() -> Self {
            Self::Warn
        }

        pub fn error() -> Self {
            Self::Error
        }

        pub fn as_tracing_level(&self) -> Option<tracing::Level> {
            use tracing::Level;

            match self {
                Self::Debug => Some(Level::DEBUG),
                Self::Error => Some(Level::ERROR),
                Self::Info => Some(Level::INFO),
                Self::Trace => Some(Level::TRACE),
                Self::Warn => Some(Level::WARN),
                Self::Off => None,
            }
        }

        pub fn as_tracing_filter(&self) -> tracing_subscriber::filter::LevelFilter {
            use tracing_subscriber::filter::LevelFilter;

            match self {
                Self::Debug => LevelFilter::DEBUG,
                Self::Error => LevelFilter::ERROR,
                Self::Info => LevelFilter::INFO,
                Self::Trace => LevelFilter::TRACE,
                Self::Warn => LevelFilter::WARN,
                Self::Off => LevelFilter::OFF,
            }
        }
    }

    impl From<LogLevel> for config::Value {
        fn from(level: LogLevel) -> Self {
            level.to_string().into()
        }
    }

    impl From<tracing::Level> for LogLevel {
        fn from(level: tracing::Level) -> Self {
            Self::from_tracing(level)
        }
    }

    unsafe impl Send for LogLevel {}

    unsafe impl Sync for LogLevel {}
}
