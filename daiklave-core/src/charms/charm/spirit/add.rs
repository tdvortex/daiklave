use super::{spirit_only::AddSpiritOnlyCharm, AddEclipseCharm};

/// A Spirit charm (which may be Eclipse or not) to be added to a character.
pub enum AddSpiritCharm {
    /// A Spirit charm with the Eclipse keyword, which may be added by Eclipse
    /// caste Solars as well as non-Exalt beings.
    Eclipse(AddEclipseCharm),
    /// A charm which can only be added to non-Exalts.
    SpiritOnly(AddSpiritOnlyCharm),
}
