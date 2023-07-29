use crate::util;
use crate::Sms;
use crate::SmsList;
use glob::glob;

pub async fn process(data: SmsList) {
    let state = &mut *data.data.write();
    util::log(&format!("attempting to load state"));
    for entry in glob("./*.txtly").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Ok(json) = util::slurp(&path.to_string_lossy()) {
                    let parsed: Sms = serde_json::from_str(&json).unwrap();
                    state.insert(
                        format!(
                            "{}",
                            &path
                                .file_name()
                                .expect("unable to decode filepath")
                                .to_string_lossy()
                                .replace(".txtly", "")
                        ),
                        parsed,
                    );
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    util::log(&format!(
        "loaded {} pending requests into memory",
        state.len()
    ));
}
