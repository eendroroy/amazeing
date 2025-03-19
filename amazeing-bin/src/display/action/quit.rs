use macroquad::input::{KeyCode, is_key_pressed};

pub(crate) fn quit_requested() -> bool {
    if is_key_pressed(KeyCode::Q) {
        println!("Quitting");
        true
    } else {
        false
    }
}
