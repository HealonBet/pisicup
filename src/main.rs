use crate::rsys_utils::get_cargo_home_dir;

mod rsys_utils;
mod sys_utils;

fn main() {
    println!("{}", get_cargo_home_dir());
}
