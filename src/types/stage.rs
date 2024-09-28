/*
    Appellation: stage <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumDiscriminants,
    strum::EnumIs,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[strum_discriminants(
    name(Stages),
    derive(
        Hash,
        Ord,
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
        strum::VariantNames
    ),
    serde(rename_all = "lowercase"),
    strum(serialize_all = "lowercase")
)]
pub enum Stage<T> {
    Before(T),
    During(T),
    After(T),
}

impl<T> Stage<T> {
    pub fn new(stage: Stages, value: T) -> Self {
        match stage {
            Stages::Before => Self::Before(value),
            Stages::During => Self::During(value),
            Stages::After => Self::After(value),
        }
    }

    pub fn before(value: T) -> Self {
        Self::Before(value)
    }

    pub fn during(value: T) -> Self {
        Self::During(value)
    }

    pub fn after(value: T) -> Self {
        Self::After(value)
    }

    pub const fn get(&self) -> &T {
        match self {
            Self::Before(value) => value,
            Self::During(value) => value,
            Self::After(value) => value,
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        match self {
            Self::Before(value) => value,
            Self::During(value) => value,
            Self::After(value) => value,
        }
    }

    pub fn into_value(self) -> T {
        match self {
            Self::Before(value) => value,
            Self::During(value) => value,
            Self::After(value) => value,
        }
    }

    pub fn stage(&self) -> Stages {
        match self {
            Self::Before(_) => Stages::Before,
            Self::During(_) => Stages::During,
            Self::After(_) => Stages::After,
        }
    }
}

impl<T> AsRef<T> for Stage<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> AsMut<T> for Stage<T> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::borrow::Borrow<T> for Stage<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> core::borrow::BorrowMut<T> for Stage<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::ops::Deref for Stage<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> core::ops::DerefMut for Stage<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
