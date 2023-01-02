use self::{equipped::ExaltEquippedWeapons, unequipped::ExaltUnequippedWeapons};

mod equipped;
mod hands;
mod unequipped;

struct ExaltWeapons<'source> {
    equipped: ExaltEquippedWeapons<'source>,
    unequipped: ExaltUnequippedWeapons<'source>,
}