use std::path::{PathBuf};
use crate::sys_utils;

pub fn get_cargo_home_dir() -> String {
    let user_home_dir = sys_utils::get_user_home_dir();
    let p = PathBuf::from_iter([user_home_dir, ".cargo".to_string()]);
    p.as_path().to_str().unwrap().to_string()
}