pub(crate) fn current_millis() -> u128 {
    std::time::UNIX_EPOCH.elapsed().unwrap().as_millis()
}
