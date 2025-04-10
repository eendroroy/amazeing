use macroquad::prelude::get_frame_time;

pub(crate) fn delay_till_next_frame(fps: f32) {
    let minimum_frame_time = 1. / fps;
    let time_to_sleep: f32;
    // let mut time_to_sleep = 0.;
    let frame_time = get_frame_time();
    if frame_time < minimum_frame_time {
        time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
    }
    // println!("Min Frame time: {}ms | Frame time: {}ms | Sleep for {}ms", minimum_frame_time, frame_time * 1000., time_to_sleep);
}
