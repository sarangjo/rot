use arboard::Clipboard;

use crate::core::rot;

pub mod core;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(rot()).unwrap();
}
