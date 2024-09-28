/*
    Appellation: opts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{build::*, deploy::*, serve::*};

mod build;
mod deploy;
mod serve;

use crate::{AsyncHandle, Context};

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
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Cmd {
    Build(BuildCmd),
    Serve(ServeCmd),
}

impl Cmd {
    pub fn build(args: BuildCmd) -> Self {
        Self::Build(args)
    }

    pub fn serve(args: ServeCmd) -> Self {
        Self::Serve(args)
    }

    #[tracing::instrument(skip(self), name = "handle", target = "cmd")]
    pub async fn handle<Ctx>(self, ctx: &mut Ctx) -> <Self as AsyncHandle<Ctx>>::Output
    where
        Self: AsyncHandle<Ctx>,
        Ctx: core::fmt::Debug,
    {
        <Self as AsyncHandle<Ctx>>::handle(self, ctx).await
    }
}


#[async_trait::async_trait]
impl AsyncHandle<Context> for Cmd {
    type Output = anyhow::Result<()>;

    async fn handle(self, ctx: &mut Context) -> Self::Output {
        match self {
            Self::Build(cmd) => {
                cmd.handle(ctx).await?;
            }
            Self::Serve(cmd) => {
                cmd.handle(ctx).await?;
            }
        }

        Ok(())
    }
}
