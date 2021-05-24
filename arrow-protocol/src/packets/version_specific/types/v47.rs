use serde::{Deserialize, Serialize};

/// Dimension type as int enum
#[repr(i32)]
#[derive(Serialize, Deserialize)]
pub enum Dimension {
    /// nehter dimension
    Nether = -1,
    /// overworld dimension
    Overworld = 0,
    /// end dimension
    End = 1,
}
