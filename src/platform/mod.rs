/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]
#[doc(inline)]
pub use self::{
    builder::PlatformBuilder, cmd::PlatformCmd, config::PlatformConfig, context::PlatformContext,
    interface::Platform,
};

mod builder;
mod config;
mod context;
mod interface;

pub mod cmd;
pub mod events;

pub(crate) mod prelude {
    pub use super::builder::*;
    pub use super::interface::*;

    pub use super::events::prelude::*;
}
