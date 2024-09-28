/*
    Appellation: serve <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{actor::Server, context::ServerContext};

mod actor;
mod context;

use tower_http::services;

fn serve_dir<S, P>(path: &str, workdir: P) -> axum::Router<S>
where
    P: AsRef<std::path::Path>,
    S: Clone + Send + Sync + 'static,
{
    axum::Router::new().nest_service(path, services::ServeDir::new(workdir))
}

fn _serve_file<S, P>(path: &str, file: P) -> axum::Router<S>
where
    P: AsRef<std::path::Path>,
    S: Clone + Send + Sync + 'static,
{
    axum::Router::new().nest_service(path, services::ServeFile::new(file))
}
