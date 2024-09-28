/*
    Appellation: deploy <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ApplicationType;

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
pub struct DeployCmd {
    #[clap(subcommand)]
    pub args: Option<DeployOpts>,
    #[clap(long, short)]
    pub kind: Option<ApplicationType>,
    #[clap(long, short)]
    pub platform: Option<String>,
    #[clap(long, short)]
    pub target: Option<String>,
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
    strum::EnumIs,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum DeployOpts {
    Wasm,
}
