//! Movement module

/// Movements the martian may perform
#[derive(Debug, PartialEq, Clone)]
pub enum Movement {
    Forward,
    Left,
    Right,
}
