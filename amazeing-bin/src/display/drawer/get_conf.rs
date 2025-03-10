use crate::context::CONTEXT;
use macroquad::prelude::Conf;

pub(crate) fn get_conf() -> Conf {
    Conf {
        window_title: CONTEXT.read().unwrap().title.clone(),
        ..Default::default()
    }
}
