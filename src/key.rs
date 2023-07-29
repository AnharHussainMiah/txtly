use crate::util;
use uuid::Uuid;

pub fn get_key() -> String {
    util::log("attempting to load the key..");
    match util::slurp("key") {
        Ok(key) => return key,
        Err(e) => {
            util::log(&format!("Error unable to find key: ({})", e));
            util::log(&format!("generating a new key!"));
            let new_key = Uuid::new_v4();
            let _ = util::dump("key", &new_key.to_string());
            return new_key.to_string();
        }
    }
}
