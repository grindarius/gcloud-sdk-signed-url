#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SignedURLScheme {
    #[default]
    V2,
    V4,
}

pub mod v2;
pub mod v4;
