use self::{equipped::MortalEquippedWeapons, unequipped::MortalUnequippedWeapons};

mod equipped;
mod hands;
mod unequipped;

pub(crate) struct MortalWeapons<'source> {
    equipped: MortalEquippedWeapons<'source>,
    unequipped: MortalUnequippedWeapons<'source>,
}