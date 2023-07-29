use crate::models::Sms;
use crate::models::SmsList;
use crate::models::Status;
use crate::util;
use crate::Invalid;
use chrono::DateTime;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::{json, with_status};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SendPayload {
    phone: String,
    message: String,
}

pub async fn handle(
    payload: SendPayload,
    data: SmsList,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = self::validate_data(&payload) {
        return Ok(with_status(json(&error), StatusCode::BAD_REQUEST));
    }
    let state = &mut *data.data.write();
    let new_request_key = Uuid::new_v4().to_string();
    let new_request_data = Sms {
        mobile: payload.phone,
        message: payload.message,
        device_id: "".to_string(),
        attempts: 0,
        added_on: DateTime::from(chrono::Local::now()),
        status: Status::New,
    };
    state.insert(new_request_key.to_string(), new_request_data.clone());
    let _ = util::dump(
        &format!("{}.txtly", new_request_key),
        &serde_json::to_string(&new_request_data).expect("error unable to serialise data"),
    );
    Ok(with_status(json(&state), StatusCode::OK))
}

fn validate_data(data: &SendPayload) -> Option<Invalid> {
    if !self::is_valid_phone(&data.phone) {
        return Some(Invalid {
            error: "phone number provided is invalid".to_string(),
        });
    }
    if data.message.len() > 160 {
        return Some(Invalid {
            error: "SMS message exceeds 160 characters".to_string(),
        });
    }
    if !self::is_valid_sms_message(&data.message) {
        return Some(Invalid {
            error: "SMS contains one or more illegal characters outside of the GSM7 spec"
                .to_string(),
        });
    }
    None
}

fn is_valid_phone(phone: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[0-9]{11}$").unwrap();
    }
    RE.is_match(phone)
}

fn is_valid_sms_message(sms: &str) -> bool {
    // TOOD: add tests to confirm that regex actually works as expected
    // https://stackoverflow.com/questions/5186702/looking-for-a-list-of-valid-characters-that-can-be-sent-in-sms-text-messages
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[A-Za-z-0-9\s\n\r\u001B\$£¥\u00A4èéùìòÇØøÆæßÉÅåÄÖÑÜ§äöñüà\u0394\u03A6\u0393\u039B\u03A9\u03A0\u03A8\u03A3\u0398\u039E\!\#\u0022\%\^\&\*\(\)\_\-\+\=\{\}\[\]\;\:\'\@\,<>\.\/\?\~\`\|¡¿]+$").unwrap();
    }
    RE.is_match(sms)
}
