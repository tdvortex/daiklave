use super::{AddEclipseCharm, noneclipse::AddNonEclipseCharm};

/// A Spirit charm (which may be Eclipse or not) to be added to a character.
pub enum AddSpiritCharm {
    Eclipse(AddEclipseCharm),
    NonEclipse(AddNonEclipseCharm),
}