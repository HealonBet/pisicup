use std::env;
#[cfg(target_os = "macos")]
pub fn get_user_home_dir() -> String {
    return env!("HOME").to_string()
}

#[cfg(target_os = "windows")]
pub fn get_user_home_dir() -> String {
    return env!("HOMEPATH").to_string()
}

#[cfg(target_os = "linux")]
pub fn get_user_home_dir() -> String {
    return env!("HOME").to_string()
}
