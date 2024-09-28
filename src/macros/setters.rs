/*
    Appellation: setters <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! setwith {

    ($($name:ident: $ty:ty),* $(,)?) => {
        $(
            set!($name: $ty);
            with!(@impl $name: $ty);
        )*
    };
}

macro_rules! set {
    (@impl $name:ident: $ty:ty) => {
        paste::paste! {
            pub fn [<set_ $name>](&mut self, $name: $ty) {
                self.$name = $name;
            }
        }
    };

    ($($name:ident: $ty:ty),* $(,)?) => {
        $(
            set!(@impl $name: $ty);
        )*
    };
}

macro_rules! with {
    (@impl $name:ident: $ty:ty) => {
        paste::paste! {
            pub fn [<with_ $name>](self, $name: $ty) -> Self {
                Self {
                    $name,
                    ..self
                }
            }
        }
    };

    ($($name:ident: $ty:ty),* $(,)?) => {
        $(
            with!(@impl $name: $ty);
        )*
    };
}
