/*
    Appellation: event <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::types::Uid;

pub type JsonMsg = Message<serde_json::Value>;

pub struct Message<T> {
    pub id: Uid,
    pub subject: String,
    pub data: T,
    pub timestamp: i64,
}

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIs,
)]
pub enum Event {
    ReqRes(ReqRes),
}

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIs,
)]
pub enum ReqRes {
    Request(String),
    Response(String),
}
pub enum PowerEvent {
    Initializing(String),
    Initialized(String),
    Running(String),
    Terminating(String),
    Stopped(String),
}
