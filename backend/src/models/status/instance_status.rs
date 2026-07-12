#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
    Destroyed,
}

impl InstanceStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Starting => "Starting",
            Self::Running => "Running",
            Self::Stopping => "Stopping",
            Self::Stopped => "Stopped",
            Self::Failed => "Failed",
            Self::Destroyed => "Destroyed",
        }
    }
}
