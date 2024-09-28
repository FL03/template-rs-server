/*
    Appellation: fmt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! display {
    (@json $T:ty) => {
        impl ::core::fmt::Display for $T {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(serde_json::to_string(self).unwrap().as_str())
            }
        }
    };
    (json($($T:ty),* $(,)?)) => {
        $(display!(@json $T);)*
    };
}
