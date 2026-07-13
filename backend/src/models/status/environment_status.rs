#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentStatus {
    Building,
    Running,
    Stopping,
    Stopped,
    Failed,
    Destroyed,
}

impl EnvironmentStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Building => "Building",
            Self::Running => "Running",
            Self::Stopping => "Stopping",
            Self::Stopped => "Stopped",
            Self::Failed => "Failed",
            Self::Destroyed => "Destroyed",
        }
    }
}
