/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{events::Event, PlatformCmd};
use crate::config::Settings;
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, mpsc};
use tokio_util::task::TaskTracker;

#[derive(Debug)]
pub struct PlatformContext {
    // a shared settings object
    pub(crate) settings: Arc<Mutex<Settings>>,
    // a channel for receiving commands from a client
    pub(crate) commands: mpsc::Receiver<PlatformCmd>,
    // a channel for sending events from the platform
    pub(crate) events: mpsc::Sender<Event>,
    // a channel for sending shutdown signals
    pub(crate) shutdown: broadcast::Sender<()>,
    //
    pub(crate) tracker: TaskTracker,
}

impl PlatformContext {
    pub fn new(
        settings: Settings,
        commands: mpsc::Receiver<PlatformCmd>,
        events: mpsc::Sender<Event>,
        shutdown: broadcast::Sender<()>,
    ) -> Self {
        Self {
            settings: Arc::new(Mutex::new(settings)),
            commands,
            events,
            shutdown,
            tracker: TaskTracker::new(),
        }
    }

    pub fn cnf(&self) -> Settings {
        self.settings.lock().unwrap().clone()
    }

    pub fn set_settings(&mut self, cnf: Settings) {
        *self.settings.lock().unwrap() = cnf;
    }

    pub fn shutdown(&self) -> Result<usize, broadcast::error::SendError<()>> {
        self.shutdown.send(())
    }

    pub fn with_tracing(&self) {
        self.cnf().init_tracing();
    }

    pub async fn event(&mut self, event: Event) -> Result<(), mpsc::error::SendError<Event>> {
        self.events.send(event).await
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(cmd) = self.commands.recv() => {
                    self.handle_cmd(cmd).await;
                }
            }
        }
    }

    async fn handle_cmd(&mut self, cmd: PlatformCmd) {
        match cmd {
            PlatformCmd::Serve => {
                self.with_tracing();
            }
            _ => {}
        }
    }
}
