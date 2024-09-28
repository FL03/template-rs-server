/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::consts::*;

pub mod consts {
    /// The name of the application.
    pub const APP_NAME: &str = "pzzld";
    /// A str constant for the localhost address.
    pub const LOCALHOST: &str = "127.0.0.1";
    /// The default port for the application.
    pub const DEFAULT_PORT: u16 = 8080;

    pub const ARTIFACTS: &str = ".artifacts";

    pub const DEFAULT_CONTEXT: &str = ".pzzld";

    pub const DEFAULT_WORKDIR: &str = "dist";

    #[allow(unused)]
    pub(crate) const ROOT: &str = env!("CARGO_MANIFEST_DIR");
}
