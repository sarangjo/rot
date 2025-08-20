use crate::core::rot;

pub mod core;

fn main() {
    match std::env::args().nth(1) {
        None => panic!("Need arg"),
        Some(a) => print!("{}", rot(a.as_str())),
    }
}
