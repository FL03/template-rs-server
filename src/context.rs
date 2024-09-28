/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::Settings;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) settings: Arc<Mutex<Settings>>,
    pub(crate) shutdown: broadcast::Sender<()>,
}

impl Context {
    pub fn new(settings: Settings, shutdown: broadcast::Sender<()>) -> Self {
        Self {
            settings: Arc::new(Mutex::new(settings)),
            shutdown,
        }
    }

    pub fn from_settings(cnf: Settings) -> (Self, broadcast::Receiver<()>) {
        let (shutdown_tx, shutdown_rx) = broadcast::channel::<()>(1);
        let ctx = Self::new(cnf, shutdown_tx);
        (ctx, shutdown_rx)
    }

    pub fn build() -> anyhow::Result<(Self, broadcast::Receiver<()>)> {
        let cnf = Settings::build()?;
        Ok(Self::from_settings(cnf))
    }
    /// Returns a clone of the current settings.
    pub fn cnf(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }
    /// Sets the current settings.
    pub fn set_settings(&mut self, cnf: Settings) {
        *self.settings.lock().unwrap() = cnf;
    }
    /// Signal the shutdown of the application
    pub fn shutdown(&self) -> Result<usize, broadcast::error::SendError<()>> {
        self.shutdown.send(())
    }
    /// Initialize tracing modules
    pub fn init_tracing(&self) {
        self.cnf().init_tracing();
    }

    pub fn with_tracing(self) -> Self {
        self.cnf().init_tracing();
        self
    }

    pub fn init(self) -> Initializer {
        Initializer { ctx: self }
    }
}

pub struct Initializer {
    ctx: Context,
}

impl Initializer {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    pub fn with_tracing(self) -> Self {
        self.ctx.settings.lock().unwrap().init_tracing();
        self
    }

    pub fn finish(self) -> Context {
        self.ctx
    }
}
