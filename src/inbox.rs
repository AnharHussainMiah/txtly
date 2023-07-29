use crate::models;
use crate::models::SmsWithId;
use crate::util;
use crate::Invalid;
use crate::Sms;
use crate::SmsList;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::http::StatusCode;
use warp::reply::{json, with_status};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InboxPayload {
    device_id: String,
}

pub async fn handle(
    payload: InboxPayload,
    data: SmsList,
) -> Result<impl warp::Reply, warp::Rejection> {
    let heartbeats = &mut *data.heartbeats.write();
    let beat = DateTime::from(chrono::Local::now());
    // add new heartbeat for current calling device
    *heartbeats.entry(payload.device_id.clone()).or_insert(beat) = beat;
    // kill all dead heartbeats
    heartbeats.retain(|_, t| (beat - *t) < chrono::Duration::seconds(5));
    let state_data = &mut *data.data.write();
    let cloned_state = state_data.clone();
    // assign a device ID to all new SMS requests
    for id in cloned_state.keys() {
        let sms = &cloned_state[id];
        if sms.device_id == "" && sms.status == models::Status::New && heartbeats.len() > 0 {
            // randomly pick a deviceId
            let random_index = (rand::random::<f32>() * heartbeats.len() as f32).floor() as usize;
            if let Some(random_device_id) = heartbeats.keys().skip(random_index).next() {
                let updated_sms = Sms {
                    mobile: sms.mobile.to_string(),
                    message: sms.message.to_string(),
                    added_on: sms.added_on,
                    attempts: 0,
                    device_id: random_device_id.to_string(),
                    status: models::Status::New,
                };
                util::log(&format!(
                    "assigned deviceID ({}) to messageID [{}]",
                    random_device_id.to_string(),
                    id
                ));
                *state_data
                    .entry(id.to_string())
                    .or_insert(updated_sms.clone()) = updated_sms.clone();
                let _ = util::dump(
                    &format!("{}.txtly", id),
                    &serde_json::to_string(&updated_sms).expect("error unable to serialise data"),
                );
            }
        }
        // check for SMS that has a deviceID AND is over XX minutes, means it hasn't been sent
        // so we can remove the device ID and let another device try send it
        
    }
    // return a SMS message to send out that is bound to a current deviceId and is NEW
    for message_id in cloned_state.keys() {
        let sms = &cloned_state[message_id];
        if sms.device_id == payload.device_id && sms.status == models::Status::New {
            let response = SmsWithId {
                id: message_id.to_string(),
                mobile: sms.mobile.to_string(),
                message: sms.message.to_string(),
                added_on: sms.added_on,
                attempts: sms.attempts,
                device_id: sms.device_id.to_string(),
                status: sms.status.clone(),
            };
            return Ok(with_status(json(&response), StatusCode::OK));
        }
    }
    Ok(with_status(
        json(&Invalid {
            error: "no messages".to_string(),
        }),
        StatusCode::NOT_FOUND,
    ))
}
