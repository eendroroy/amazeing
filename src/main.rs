use amazeing::app::run;
use macroquad::prelude::Conf;

#[macroquad::main(window_config())]
async fn main() {
    run().await;
}

fn window_config() -> Conf {
    Conf {
        window_title: "Amazeing".to_string(),
        high_dpi: true,
        sample_count: 10,
        window_resizable: false,
        // Disable vsync so the frame-pacing loop in `delay_till_next_frame`
        // can target any FPS value — including values well above the typical
        // monitor refresh rate of 60/144 Hz.
        platform: macroquad::miniquad::conf::Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}
