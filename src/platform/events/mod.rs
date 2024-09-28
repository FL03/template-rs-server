/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::event::Event;

mod event;

pub(crate) mod prelude {
    pub use super::event::*;
}

pub(crate) type MpscEventTx = tokio::sync::mpsc::Sender<Event>;
pub(crate) type MpscEventRx = tokio::sync::mpsc::Receiver<Event>;
