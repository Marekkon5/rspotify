//! Offset object

use chrono::Duration;

/// Offset object
#[derive(Clone, Debug, PartialEq)]
pub enum Offset {
    Position(Duration),
    Uri(String),
}
