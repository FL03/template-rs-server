/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod handle;

pub(crate) mod prelude {
    pub use super::handle::*;
}
