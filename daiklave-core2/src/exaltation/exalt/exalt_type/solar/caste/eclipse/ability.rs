use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum EclipseAbility {
    Bureaucracy,
    Larceny,
    Linguistics,
    Occult,
    Presence,
    Ride,
    Sail,
    Socialize,
}
