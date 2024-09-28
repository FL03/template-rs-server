/*
    Appellation: server <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use puzzled::{cmd::Cli, Context, Settings};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (shutdown_tx, _shutdown_rx) = broadcast::channel::<()>(1);
    // build the settings
    let cnf = dbg!(Settings::build()?);
    // initialize a new context
    let mut ctx = Context::new(cnf, shutdown_tx)
        .init()
        .with_tracing()
        .finish();
    // parse the command line
    let cli = dbg!(Cli::new());
    // handle the command
    cli.handle(&mut ctx).await
}
