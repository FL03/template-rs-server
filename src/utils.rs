/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

///
#[tracing::instrument(level = "trace", name = "shutdown", target = "platform")]
pub async fn graceful_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to gracefully shutdown the platform");
    tracing::trace!("Signal received; shutting down the platform and related services...");
}

/// [systime] is a utilitarian function that returns the current system time in milliseconds.
#[inline]
pub fn systime() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[inline]
pub fn std_time() -> core::time::Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
}

pub fn timestamp() -> i64 {
    chrono::Local::now().timestamp()
}

pub(crate) fn default_ip() -> String {
    core::net::IpAddr::V4(core::net::Ipv4Addr::LOCALHOST).to_string()
}

pub(crate) fn default_port() -> u16 {
    crate::DEFAULT_PORT
}
