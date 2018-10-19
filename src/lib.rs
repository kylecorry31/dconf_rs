use std::process::{Command, Output};

/// Sets a boolean value through dconf
pub fn set_boolean(key: &str, value: bool) -> Option<()> {
	set(key, &format!("{}", value))
}

/// Gets a boolean value through dconf
pub fn get_boolean(key: &str) -> Option<bool> {
	Some(get(key)? == "true")
}

/// Sets a string value through dconf
pub fn set_string(key: &str, value: &str) -> Option<()> {
    set(key, &format!("'{}'", value))
}

/// Gets a string value through dconf
pub fn get_string(key: &str) -> Option<String> {
    get(key)
}

/// Sets an int value through dconf
pub fn set_int(key: &str, value: i32) -> Option<()> {
    set(key, &format!("{}", value))
}

/// Gets an int value through dconf
pub fn get_int(key: &str) -> Option<i32> {
    Some(get(key)?.parse::<i32>().unwrap())
}

/// Sets a uint value through dconf
pub fn set_uint(key: &str, value: u32) -> Option<()> {
    set(key, &format!("{}", value))
}

/// Gets a uint value through dconf
pub fn get_uint(key: &str) -> Option<u32> {
    Some(get(key)?.parse::<u32>().unwrap())
}

/// Sets a double value through dconf
pub fn set_double(key: &str, value: f64) -> Option<()> {
    set(key, &format!("{}", value))
}

/// Gets a double value through dconf
pub fn get_double(key: &str) -> Option<f64> {
    Some(get(key)?.parse::<f64>().unwrap())
}


// Helpers
fn get(key: &str) -> Option<String> {
    let mut cmd = Command::new("dconf");
	cmd.args(&["read", key]);
	Some(get_stdout(cmd.output().unwrap()))
}

fn set(key: &str, value: &str) -> Option<()>{
    let mut cmd = Command::new("dconf");
	cmd.args(&["write", key, value]);
	match cmd.output() {
		Ok(_) => Some(()),
		Err(_) => None,
	}
}

fn get_stdout(output: Output) -> String {
	let vs = output.stdout;
	String::from_utf8(vs).unwrap().replace("\'", "").replace("\n", "")
}
