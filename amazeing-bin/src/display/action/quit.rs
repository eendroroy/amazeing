use macroquad::input::{is_key_pressed, KeyCode};

pub(crate) fn quit_requested() -> bool {
    if is_key_pressed(KeyCode::Q) {
        println!("Quitting");
        true
    } else {
        false
    }
}
