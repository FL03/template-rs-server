/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::time::Duration;

/// [Timestamp] is a generic type used to represent a timestamp.
///
/// The timestamp considers the standard timestamp to be the
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Timestamp<T = u64>(pub T);

impl<T> Timestamp<T> {
    pub fn new(ts: T) -> Self {
        Self(ts)
    }
    /// Get an immutable reference to the current timestamp.
    pub fn as_ref(&self) -> &T {
        &self.0
    }
    /// Get a mutable reference to the current timestamp.
    pub fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// Get the current timestamp.
    pub fn get(self) -> T {
        self.0
    }
    /// Replace the current timestamp with a new one.
    pub fn replace(&mut self, ts: T) -> T {
        core::mem::replace(&mut self.0, ts)
    }
    /// Set the current timestamp to a new value.
    pub fn set(&mut self, ts: T) {
        self.0 = ts;
    }
    /// Take the current timestamp and replace it with the default value.
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(&mut self.0)
    }
}

/// Standard timestamp in seconds.
impl Timestamp<u64> {
    pub fn now() -> Self {
        Self(crate::std_time().as_secs())
    }
}

/// Standard timestamp in milliseconds.
impl Timestamp<u128> {
    pub fn now() -> Self {
        Self(crate::std_time().as_millis())
    }
}

/// [`chrono`] timestamp
impl Timestamp<i64> {
    pub fn now() -> Self {
        Self(chrono::Local::now().timestamp())
    }

    pub fn from_chrono<Tz: chrono::TimeZone>(ts: chrono::DateTime<Tz>) -> Self {
        Self(ts.timestamp())
    }
}

impl<T> AsRef<T> for Timestamp<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Timestamp<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::borrow::Borrow<T> for Timestamp<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> core::borrow::BorrowMut<T> for Timestamp<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::ops::Deref for Timestamp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Timestamp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> core::fmt::Display for Timestamp<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Duration> for Timestamp<u64> {
    fn from(dur: Duration) -> Self {
        Self(dur.as_secs())
    }
}

impl From<Duration> for Timestamp<u128> {
    fn from(dur: Duration) -> Self {
        Self(dur.as_millis())
    }
}

impl<Tz> From<chrono::DateTime<Tz>> for Timestamp<i64>
where
    Tz: chrono::TimeZone,
{
    fn from(ts: chrono::DateTime<Tz>) -> Self {
        Self(ts.timestamp())
    }
}
