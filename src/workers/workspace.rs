/*
    Appellation: workspace <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::types::Uid;
pub struct Workspace {
    pub id: Uid,
    pub description: String,
    pub name: String,
    pub path: String,

    pub services: Vec<String>,
}
