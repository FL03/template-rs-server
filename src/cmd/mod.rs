/*
    Appellation: cmd <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{interface::Cli, opts::Cmd};

mod interface;

pub mod opts;

pub(crate) mod prelude {
    pub use super::interface::Cli;
    pub use super::opts::*;
}

use crate::config::Scope;
use crate::workers::serve::Server;

display! {
    json(
        Cli,
        opts::BuildCmd,
        opts::ServeCmd,
    )
}

pub(crate) async fn _serve(addr: core::net::SocketAddr, scope: Scope) -> crate::Result {
    let server = Server::new(addr, scope);
    tokio::join!(server.serve());
    Ok(())
}
