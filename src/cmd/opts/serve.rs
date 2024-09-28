/*
    Appellation: serve <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::workers::serve::Server;
use crate::{AsyncHandle, Context};

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Parser,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct ServeCmd {
    #[clap(subcommand)]
    pub args: Option<ServeOpts>,
    #[clap(long, short = 'H', default_value_t = core::net::Ipv4Addr::LOCALHOST.to_string())]
    pub host: String,
    #[clap(long, short, default_value_t = 8080)]
    pub port: u16,
    #[clap(long, short)]
    pub workdir: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Subcommand,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum ServeOpts {
    Run {
        #[clap(long, short)]
        prefix: Option<String>,
    },
}

impl ServeCmd {
    pub fn new() -> Self {
        clap::Parser::parse()
    }

    pub fn args(&self) -> Option<&ServeOpts> {
        self.args.as_ref()
    }

    pub fn addr(&self) -> core::net::SocketAddr {
        format!("{}:{}", self.host, self.port).parse().unwrap()
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    #[tracing::instrument(skip(self), name = "handle", target = "serve")]
    pub async fn handle<Ctx>(self, ctx: &mut Ctx) -> <Self as AsyncHandle<Ctx>>::Output
    where
        Self: AsyncHandle<Ctx>,
        Ctx: core::fmt::Debug,
    {
        <Self as AsyncHandle<Ctx>>::handle(self, ctx).await
    }
}

#[async_trait::async_trait]
impl AsyncHandle<Context> for ServeCmd {
    type Output = anyhow::Result<()>;

    async fn handle(self, ctx: &mut Context) -> Self::Output {
        let addr = self.addr();
        let mut config = ctx.cnf();
        // update the workdir; if it was set
        config.set_some_workdir(self.workdir);
        // update the context with the new settings
        ctx.set_settings(config.clone());
        // create a new server instance
        let server = Server::new(addr, config.scope);
        // start the server
        tokio::join!(server.serve());
        Ok(())
    }
}
