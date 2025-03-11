#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    None { name: String },
    Generate,
    Visualize,
    Modify,
    Simulate,
    Realtime,
}
