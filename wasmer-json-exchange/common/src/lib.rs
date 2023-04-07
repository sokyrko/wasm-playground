use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub message: String,
    pub number: u64,
}

impl Request {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(json: &[u8]) -> Self {
        serde_json::from_slice(json).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub message: String,
    pub number: u64,
}

impl Response {
    pub fn from_json(json: &[u8]) -> Self {
        serde_json::from_slice(json).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
