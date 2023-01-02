use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HearthstoneCategory {
    Air,
    Earth,
    Fire,
    Water,
    Wood,
    Solar,
    Sidereal,
    Lunar,
    Abyssal,
}