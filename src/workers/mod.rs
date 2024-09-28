/*
    Appellation: workers <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]
#[doc(inline)]
pub use self::{config::WorkerConfig, workspace::Workspace};

mod config;
mod workspace;

pub mod build;
pub mod serve;

pub mod prelude {
    pub use super::config::*;
    pub use super::workspace::*;
}
