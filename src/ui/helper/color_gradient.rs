use macroquad::prelude::Color;

pub(crate) fn gradient(c1: Color, c2: Color, steps: usize) -> Vec<Color> {
    if steps <= 1 {
        return vec![c2];
    }

    let rs = (c2.r - c1.r) / (steps as f32 - 1.0);
    let gs = (c2.g - c1.g) / (steps as f32 - 1.0);
    let bs = (c2.b - c1.b) / (steps as f32 - 1.0);
    let as_step = (c2.a - c1.a) / (steps as f32 - 1.0);

    (0..steps)
        .map(|i| {
            let t = i as f32;
            Color::new(c1.r + rs * t, c1.g + gs * t, c1.b + bs * t, c1.a + as_step * t)
        })
        .collect()
}
