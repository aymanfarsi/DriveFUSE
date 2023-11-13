#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MountingOptions {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MountingOption {
    Mount,
    Unmount,
}