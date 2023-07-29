use chrono::serde::ts_seconds;
use chrono::Utc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sms {
    pub mobile: String,
    pub message: String,
    pub status: Status,
    pub device_id: String,
    #[serde(with = "ts_seconds")]
    pub added_on: chrono::DateTime<Utc>,
    pub attempts: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmsWithId {
    pub id: String,
    pub mobile: String,
    pub message: String,
    pub status: Status,
    pub device_id: String,
    #[serde(with = "ts_seconds")]
    pub added_on: chrono::DateTime<Utc>,
    pub attempts: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Status {
    New,
    Sent,
    Error,
    Timeout,
}

#[derive(Clone)]
pub struct SmsList {
    pub data: Arc<RwLock<HashMap<String, Sms>>>,
    pub heartbeats: Arc<RwLock<HashMap<String, chrono::DateTime<Utc>>>>,
}

impl SmsList {
    pub fn new() -> Self {
        SmsList {
            data: Arc::new(RwLock::new(HashMap::new())),
            heartbeats: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
