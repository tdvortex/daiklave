mod id;
pub use id::SolarCharmId;

use serde::{Serialize, Deserialize};

/// A Solar charm. 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarCharm;