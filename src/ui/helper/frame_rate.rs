use crate::ui::helper::current_millis;

pub(crate) fn delay_till_next_frame(current_frame_start_time: u128, fps: f32) {
    let current_frame_time = (current_millis() - current_frame_start_time) as f32;
    let minimum_frame_time = (1. / fps) * 1000.;
    let time_to_sleep: f32;
    if current_frame_time < minimum_frame_time {
        time_to_sleep = minimum_frame_time - current_frame_time;
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
    }
    // println!("Min: {}ms | Current: {}ms | Sleep: {}ms", minimum_frame_time, current_frame_time, time_to_sleep);
}
