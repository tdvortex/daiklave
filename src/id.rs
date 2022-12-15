use std::ops::Deref;

use serde::{Serialize, Deserialize};

/// A unique identifier for a particular resource.
/// 
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Id {
    Database(i32),
    Placeholder(i32),
}

impl Deref for Id {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Database(i) | Self::Placeholder(i) => &i,
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::Placeholder(0)
    }
}

impl Id {
    pub fn is_placeholder(&self) -> bool {
        match self {
            Self::Placeholder(_) => true,
            _ => false,
        }
    }
}