/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::ValueEnum,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ApplicationType {
    Desktop,
    Mobile,
    #[default]
    Web,
}

impl ApplicationType {
    pub fn desktop() -> Self {
        Self::Desktop
    }

    pub fn mobile() -> Self {
        Self::Mobile
    }

    pub fn web() -> Self {
        Self::Web
    }
}
