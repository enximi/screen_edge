#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserOperation {
    ScrollUp,
    ScrollDown,
    MiddlePress,
    MiddleRelease,
    Touch,
}
