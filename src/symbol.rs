use std::fmt;
use strum::EnumIter;

use serde::Serialize;

#[derive(Serialize, Copy, Clone, EnumIter, Debug)]
#[allow(dead_code)]
pub enum Symbol {
    // Vertical
    // Crypto,
    ML,

    // Product etc
    Docker,
    NEAR,
    // Notion,
    // Python,
    Rust,

    // Project
    NotionAutomation,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::ML => write!(f, "🎲"),
            Symbol::Docker => write!(f, "🐋"),
            Symbol::NEAR => write!(f, "🔗"),
            Symbol::NotionAutomation => write!(f, "📦"),
            Symbol::Rust => write!(f, "⚙️"),
        }
    }
}
