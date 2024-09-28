/*
    Appellation: cnf <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Configuration module for the server.
#[doc(inline)]
pub use self::{kinds::*, settings::*};

mod settings;

pub mod kinds {
    #[doc(inline)]
    pub use self::{logger::*, network::*, scope::*};

    mod logger;
    mod network;
    mod scope;
}

pub(crate) mod prelude {
    pub use super::kinds::*;
    pub use super::settings::*;
}

display! {
    json(
        Settings,
        LoggerConfig,
        NetworkConfig,
    )
}
