use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn current_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
