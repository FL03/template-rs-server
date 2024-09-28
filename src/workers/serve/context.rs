/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::Scope;
use core::net::SocketAddr;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct ServerContext {
    pub addr: SocketAddr,
    pub scope: Scope,
}

impl ServerContext {
    pub fn new(addr: SocketAddr, scope: Scope) -> Self {
        Self { addr, scope }
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub const fn scope(&self) -> &Scope {
        &self.scope
    }

    setwith! {
        addr: SocketAddr,
        scope: Scope,
    }

    pub async fn listen(&self) -> tokio::net::TcpListener {
        tokio::net::TcpListener::bind(&self.addr).await.unwrap()
    }
}
