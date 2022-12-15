use crate::charms::Spell;

pub enum SorcererCircle {
    Terrestrial,
    Celestial,
    Solar,
}

pub struct SorcererTraits {
    circle: SorcererCircle,
    initiation: String,
    shaping_rituals: Vec<String>,
    control_spell: Spell,
    other_spells: Vec<Spell>,
}