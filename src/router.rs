use crate::util;
use crate::SmsList;
use chrono::DateTime;
use std::thread;
use std::time::Duration;

pub async fn schedule(data: SmsList) {
    /*----------------------------------------------------------------------------------------------

    1. remove dead heartbeats
    2. for each SMS that is "new" + no deviceD:
        - pick a randome deviceID from the heartbeat
        - update the sms with new deviceID (if one comes back from last step)
    ----------------------------------------------------------------------------------------------*/
    loop {
        let state = &mut *data.data.write();
        let heartbeats = &mut *data.heartbeats.write();
        let now = DateTime::from(chrono::Local::now());
        heartbeats.retain(|_, t| (now - *t) > chrono::Duration::seconds(5));

        format!("{:?}", heartbeats);

        drop(state);
        drop(heartbeats);

        //let keys = state.clone();
        // for id in keys.keys() {
        //     let sms = &state[id];
        //     if sms.status == models::Status::New && sms.attempts > 3 {
        //         state.remove(id);
        //         let _ = util::shove(&format!("{}.txtly", id), &format!("{}.timeout", id));
        //     }
        // }
        thread::sleep(Duration::from_secs(5));
    }
}