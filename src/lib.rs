use std::process::{Command, Output};
use std::io::Result;

/// Sets a boolean value through dconf
pub fn set_boolean(key: &str, value: bool) -> Result<()> {
	set(key, &format!("{}", value))
}

/// Gets a boolean value through dconf
pub fn get_boolean(key: &str) -> Result<bool> {
	Ok(get(key)? == "true")
}

/// Sets a string value through dconf
pub fn set_string(key: &str, value: &str) -> Result<()> {
    set(key, &format!("'{}'", value))
}

/// Gets a string value through dconf
pub fn get_string(key: &str) -> Result<String> {
    get(key)
}

/// Sets an int value through dconf
pub fn set_int(key: &str, value: i32) -> Result<()> {
    set(key, &format!("{}", value))
}

/// Gets an int value through dconf
pub fn get_int(key: &str) -> Result<i32> {
    Ok(get(key)?.parse::<i32>().unwrap())
}

/// Sets a uint value through dconf
pub fn set_uint(key: &str, value: u32) -> Result<()> {
    set(key, &format!("{}", value))
}

/// Gets a uint value through dconf
pub fn get_uint(key: &str) -> Result<u32> {
    Ok(get(key)?.parse::<u32>().unwrap())
}

/// Sets a double value through dconf
pub fn set_double(key: &str, value: f64) -> Result<()> {
    set(key, &format!("{}", value))
}

/// Gets a double value through dconf
pub fn get_double(key: &str) -> Result<f64> {
    Ok(get(key)?.parse::<f64>().unwrap())
}


// Helpers
fn get(key: &str) -> Result<String> {
    let mut cmd = Command::new("dconf");
	cmd.args(&["read", key]);
	Ok(get_stdout(cmd.output()?))
}

fn set(key: &str, value: &str) -> Result<()>{
    let mut cmd = Command::new("dconf");
	cmd.args(&["write", key, value]);
	match cmd.output() {
		Ok(_) => Ok(()),
		Err(e) => Err(e),
	}
}

fn get_stdout(output: Output) -> String {
	let vs = output.stdout;
	String::from_utf8(vs).unwrap().replace("\'", "").replace("\n", "")
}
