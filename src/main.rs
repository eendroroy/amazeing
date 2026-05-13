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
        ..Default::default()
    }
}
