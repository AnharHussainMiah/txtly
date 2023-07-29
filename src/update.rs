use crate::models::{Sms, Status};
use crate::util;
use crate::Invalid;
use crate::SmsList;
use serde::{Deserialize, Serialize};
use serde_json;
use warp::http::StatusCode;
use warp::reply::{json, with_status};
use chrono::DateTime;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdatePayload {
    device_id: String,
    message_id: String,
    status: MessageStatus,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MessageStatus {
    Sent,
    Error,
}

pub async fn handle(
    payload: UpdatePayload,
    data: SmsList,
) -> Result<impl warp::Reply, warp::Rejection> {
    let state = &mut *data.data.write();
    let error_message = String::new();
    match payload.status {
        MessageStatus::Sent => {
            if let Some(sms) = state.get(&payload.message_id) {
                if sms.device_id == payload.device_id {
                    state.remove(&payload.message_id);
                    let _ = util::boink(&format!("{}.txtly", payload.message_id));
                }
            }
        }
        MessageStatus::Error => {
            if let Some(sms) = state.get(&payload.message_id) {
                if sms.device_id == payload.device_id {
                    let sms = sms.clone();
                    if sms.attempts > 5 {
                        state.remove(&payload.message_id);
                        let _ = util::dump(
                            &format!("{}.timeout", &payload.message_id),
                            &serde_json::to_string(&sms).expect("error unable to serialise data"),
                        );
                    } else {
                        let updated_sms = Sms {
                            mobile: sms.mobile,
                            message: sms.message,
                            added_on: DateTime::from(chrono::Local::now()),
                            attempts: sms.attempts + 1,
                            device_id: "".to_string(),
                            status: Status::New,
                        };
                        *state
                            .entry(payload.message_id.to_string())
                            .or_insert(updated_sms.clone()) = updated_sms.clone();
                        let _ = util::dump(
                            &format!("{}.txtly", payload.message_id),
                            &serde_json::to_string(&updated_sms)
                                .expect("error unable to serialise data"),
                        );
                    }
                }
            }
        }
    }
    // TODO: sort the return status etc
    Ok(with_status(
        json(&Invalid {
            error: "not implemented".to_string(),
        }),
        StatusCode::BAD_REQUEST,
    ))
}
