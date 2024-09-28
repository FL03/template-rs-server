/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::types::NetAddr;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct NetworkConfig {
    #[serde(default)]
    pub(crate) address: Vec<NetAddr>,
    #[serde(default)]
    pub(crate) basepath: Option<String>,
    #[serde(default)]
    pub(crate) max_connections: Option<u16>,
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) open: bool,
    #[serde(default = "crate::default_port")]
    pub(crate) port: u16,
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self {
            address: Vec::new(),
            basepath: None,
            max_connections: None,
            name: None,
            open: false,
            port: crate::DEFAULT_PORT,
        }
    }

    pub fn address(&self) -> &[NetAddr] {
        &self.address
    }

    pub fn max_connections(&self) -> Option<u16> {
        self.max_connections
    }

    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("server")
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn with_basepath(self, basepath: impl ToString) -> Self {
        Self {
            basepath: Some(basepath.to_string()),
            ..self
        }
    }

    pub fn with_max_connections(self, max_connections: u16) -> Self {
        Self {
            max_connections: Some(max_connections),
            ..self
        }
    }

    pub fn with_name(self, name: impl ToString) -> Self {
        Self {
            name: Some(name.to_string()),
            ..self
        }
    }

    pub fn with_port(self, port: u16) -> Self {
        Self { port, ..self }
    }

    pub fn append_address(&mut self, address: NetAddr) {
        self.address.push(address);
    }

    pub fn push_address(&mut self, address: NetAddr) {
        self.address.push(address);
    }

    pub fn set_address<I>(&mut self, address: I)
    where
        I: IntoIterator<Item = NetAddr>,
    {
        self.address = Vec::from_iter(address);
    }

    pub fn set_basepath(&mut self, basepath: impl ToString) {
        self.basepath = Some(basepath.to_string());
    }

    pub fn set_max_connections(&mut self, max_connections: u16) {
        self.max_connections = Some(max_connections);
    }

    pub fn set_name(&mut self, name: impl ToString) {
        self.name = Some(name.to_string());
    }

    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self::new()
    }
}
