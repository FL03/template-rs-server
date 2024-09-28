/*
    Appellation: power <module>
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
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Power {
    #[default]
    Off = 0,
    On = 1,
}

impl Power {
    pub fn off() -> Self {
        Self::Off
    }

    pub fn on() -> Self {
        Self::On
    }
}

impl From<Power> for usize {
    fn from(p: Power) -> Self {
        p as usize
    }
}

impl From<usize> for Power {
    fn from(i: usize) -> Self {
        use strum::EnumCount;
        match i % Self::COUNT as usize {
            0 => Self::Off,
            _ => Self::On,
        }
    }
}

impl From<bool> for Power {
    fn from(b: bool) -> Self {
        if b {
            Self::On
        } else {
            Self::Off
        }
    }
}

impl From<Power> for bool {
    fn from(p: Power) -> Self {
        match p {
            Power::Off => false,
            Power::On => true,
        }
    }
}
