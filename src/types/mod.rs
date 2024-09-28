/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # types
//!
//! The `types` module provides a set of types and traits that are used throughout the application.
#![allow(unused)]
#[doc(inline)]
pub use self::{
    application::ApplicationType,
    environment::Environment,
    mode::Mode,
    netaddr::NetAddr,
    power::Power,
    stage::{Stage, Stages},
};

pub mod application;
pub mod environment;
pub mod mode;
pub mod netaddr;
pub mod power;
pub mod stage;
pub mod timestamp;

pub(crate) mod prelude {
    pub use super::application::*;
    pub use super::environment::*;
    pub use super::mode::*;
    pub use super::netaddr::*;
    pub use super::power::*;
    pub use super::stage::*;
    pub use super::timestamp::*;
    pub use super::{BoxError, BoxResult, Result};
}

/// A type alias for `Box<dyn core::error::Error + Send + Sync>`.
pub type BoxError = Box<dyn core::error::Error + core::marker::Send + core::marker::Sync>;
/// A type alias for `Result<T, BoxError>`.
pub type BoxResult<T> = core::result::Result<T, BoxError>;
///
pub type Result<T = ()> = core::result::Result<T, crate::Error>;

/// A type alias for [Sender<T>](tokio::sync::broadcast::Sender).
pub(crate) type PowerTx = tokio::sync::broadcast::Sender<()>;
/// A type alias for [`u32`] used to identify an object
pub(crate) type Uid = u32;
