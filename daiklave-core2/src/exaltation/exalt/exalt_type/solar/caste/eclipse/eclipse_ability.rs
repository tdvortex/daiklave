use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub(crate) enum EclipseAbility {
    Bureaucracy,
    Larceny,
    Linguistics,
    Occult,
    Presence,
    Ride,
    Sail,
    Socialize,
}