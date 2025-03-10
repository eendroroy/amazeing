use macroquad::prelude::Conf;
use crate::context::SOLVER_CONTEXT;

pub(crate) fn get_conf() -> Conf {
    Conf {
        window_title: SOLVER_CONTEXT.read().unwrap().title.clone(),
        ..Default::default()
    }
}