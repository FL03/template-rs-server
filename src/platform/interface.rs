/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::events::MpscEventTx as Events;
use super::{PlatformCmd, PlatformContext};
use crate::Settings;
use core::borrow::Borrow;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};

type Commands = tokio::sync::mpsc::Receiver<PlatformCmd>;

pub struct Platform {
    ctx: Arc<PlatformContext>,
}

impl Platform {
    pub fn new(
        settings: Settings,
        commands: Commands,
        events: Events,
        shutdown: broadcast::Sender<()>,
    ) -> Self {
        let ctx = PlatformContext::new(settings, commands, events, shutdown);

        Self { ctx: Arc::new(ctx) }
    }

    pub fn from_context(ctx: PlatformContext) -> Self {
        Self { ctx: Arc::new(ctx) }
    }

    pub fn shutdown(&self) -> Result<usize, broadcast::error::SendError<()>> {
        self.ctx.shutdown()
    }

    async fn handle_cmd(&mut self, args: PlatformCmd) {
        match args {
            PlatformCmd::Serve => {
                self.ctx.cnf().init_tracing();
            }
            _ => {}
        }
    }
}
