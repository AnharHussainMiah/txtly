mod inbox;
mod key;
mod logo;
mod models;
mod reload;
mod router;
mod send;
mod update;
mod util;

use crate::inbox::InboxPayload;
use crate::models::Sms;
use crate::models::SmsList;
use crate::send::SendPayload;
use crate::update::UpdatePayload;
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use warp::Filter;

const VERSION: &str = "0.1.0";

lazy_static! {
    static ref KEY: String = key::get_key();
}

#[tokio::main]
async fn main() {
    // https://blog.logrocket.com/building-rest-api-rust-warp/
    let data = models::SmsList::new();
    let data2 = data.clone();
    logo::draw(&VERSION);
    reload::process(data.clone()).await;
    println!("");
    println!("To pair a new TXTLY client scan the QR Code below:");
    println!("");
    qr2term::print_qr(KEY.to_string()).unwrap();
    let api_key = warp::header::exact("x-api-key", &KEY);
    let sms_data = warp::any().map(move || data.clone());

    let get_index = warp::get()
        .and(warp::path::end())
        .map(|| format!("TXTLY is running.."));

    let post_send = warp::post()
        .and(warp::path("send"))
        .and(warp::path::end())
        .and(api_key)
        .and(self::extract_json_of::<SendPayload>())
        .and(sms_data.clone())
        .and_then(send::handle);

    let post_update = warp::post()
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(api_key)
        .and(self::extract_json_of::<UpdatePayload>())
        .and(sms_data.clone())
        .and_then(update::handle);

    let post_inbox = warp::post()
        .and(warp::path("inbox"))
        .and(warp::path::end())
        .and(api_key)
        .and(self::extract_json_of::<InboxPayload>())
        .and(sms_data.clone())
        .and_then(inbox::handle);

    let routes = get_index.or(post_send).or(post_update).or(post_inbox);
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

pub fn extract_json_of<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Invalid {
    error: String,
}
