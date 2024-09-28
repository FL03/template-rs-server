/*
    Appellation: puzzled <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # puzzled
//!
//! `puzzled` is a library for creating and managing a network of nodes.
#[doc(inline)]
pub use self::{
    config::Settings, context::Context, error::Error, primitives::*, traits::prelude::*,
    types::prelude::*, utils::*,
};

#[macro_use]
pub(crate) mod macros;
pub(crate) mod primitives;
pub(crate) mod utils;

pub mod cmd;
pub mod config;
pub mod context;
pub mod error;
pub mod platform;
pub mod traits;
pub mod types;
pub mod workers;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::cmd::prelude::*;
    pub use super::config::prelude::*;
    pub use super::context::Context;
    pub use super::error::Error;
    pub use super::platform::prelude::*;
    pub use super::traits::prelude::*;
    pub use super::types::prelude::*;
    pub use super::workers::prelude::*;

    pub use super::consts::*;
}
