#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    None,
    Generate,
    Visualize,
    Modify,
    Simulate,
    Realtime,
}
