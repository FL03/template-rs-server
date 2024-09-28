/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{serve_dir, ServerContext};
use crate::config::Scope;
use core::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;

/// [Server] is a
///
///
pub struct Server {
    inner: Arc<ServerContext>,
}

impl Server {
    /// Create a new instance of [Server]
    pub fn new(addr: SocketAddr, scope: Scope) -> Self {
        let ctx = ServerContext::new(addr, scope);
        Self {
            inner: Arc::new(ctx),
        }
    }
    /// Listen on the configured address
    #[tracing::instrument(skip(self), name = "listen", target = "server")]
    pub async fn listen(&self) -> tokio::net::TcpListener {
        tracing::debug!("listening on http://{}", self.addr());
        tokio::net::TcpListener::bind(self.addr()).await.unwrap()
    }
    /// serve the resources on the configured address
    #[tracing::instrument(skip(self), name = "serve", target = "server")]
    pub async fn serve(self) {
        let listener = self.listen().await;
        axum::serve(listener, self.router())
            .with_graceful_shutdown(crate::graceful_shutdown())
            .await
            .unwrap()
    }

    fn router(&self) -> axum::Router {
        use tower_http::trace::TraceLayer;
        let workdir = self.scope().as_path();
        axum::Router::new()
            .merge(serve_dir("/", workdir))
            .layer(TraceLayer::new_for_http())
    }
}

impl core::ops::Deref for Server {
    type Target = ServerContext;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
