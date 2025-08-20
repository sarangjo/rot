use arboard::Clipboard;

use crate::core::rot;

pub mod core;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();

    match std::env::args().nth(1) {
        None => panic!("Need arg"),
        Some(a) => clipboard.set_text(rot(a.as_str())).unwrap(),
    }
}
