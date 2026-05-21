use macroquad::miniquad::conf::Icon;

const ICON_16: &[u8] = include_bytes!("icon_16x16.png");
const ICON_32: &[u8] = include_bytes!("icon_32x32.png");
const ICON_64: &[u8] = include_bytes!("icon_64x64.png");

fn decode_rgba<const N: usize>(bytes: &[u8], expected_side: u32) -> Option<[u8; N]> {
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    if image.width() != expected_side || image.height() != expected_side {
        return None;
    }
    image.into_raw().try_into().ok()
}

pub(crate) fn window_icon() -> Option<Icon> {
    Some(Icon {
        small: decode_rgba::<{ 16 * 16 * 4 }>(ICON_16, 16)?,
        medium: decode_rgba::<{ 32 * 32 * 4 }>(ICON_32, 32)?,
        big: decode_rgba::<{ 64 * 64 * 4 }>(ICON_64, 64)?,
    })
}
