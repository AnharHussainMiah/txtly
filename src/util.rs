extern crate chrono;

use self::chrono::Local;
use std::error::Error;
use std::fs;

pub fn log(msg: &str) {
    let date = Local::now();
    println!("[{}] ==> {}", date.format("%Y-%m-%d %H:%M:%S"), msg);
}

pub fn slurp(f: &str) -> Result<String, Box<dyn Error>> {
    let data: String = fs::read_to_string(&f.to_string())?.parse()?;
    Ok(data)
}

pub fn dump(f: &str, c: &str) -> Result<(), Box<dyn Error>> {
    fs::write(f, c)?;
    Ok(())
}

pub fn boink(f: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_file(f)?;
    Ok(())
}

pub fn shove(from: &str, to: &str) -> Result<(), Box<dyn Error>> {
    fs::rename(from, to)?;
    Ok(())
}
