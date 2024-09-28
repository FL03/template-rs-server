/*
    Appellation: server_addr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct NetAddr {
    #[serde(default = "crate::default_ip")]
    pub host: String,
    #[serde(default = "crate::default_port")]
    pub port: u16,
}

impl NetAddr {
    pub fn new(host: impl ToString, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }

    pub fn localhost(port: u16) -> Self {
        Self::new(crate::default_ip(), port)
    }

    pub fn as_socket_addr(&self) -> core::net::SocketAddr {
        self.to_string().parse().unwrap()
    }

    pub fn ip(&self) -> core::net::IpAddr {
        self.as_socket_addr().ip()
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Default for NetAddr {
    fn default() -> Self {
        Self::localhost(crate::DEFAULT_PORT)
    }
}

impl core::fmt::Display for NetAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{host}:{port}", host = self.host, port = self.port)
    }
}

impl core::str::FromStr for NetAddr {
    type Err = Box<dyn core::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let host: core::net::IpAddr = parts.next().unwrap().parse()?;
        let port = parts.next().unwrap().parse()?;
        Ok(Self {
            host: host.to_string(),
            port,
        })
    }
}
