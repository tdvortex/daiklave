use crate::exaltation::ExaltationSorcery;

use super::spell::Spell;

/// The spells known by a sorcerer.
pub struct Spells<'view, 'source>(pub(crate) ExaltationSorcery<'view, 'source>);

impl<'view, 'source> Spells<'view, 'source> {
    /// Gets a specific spell by its name. The second parameter returned
    /// indicates if it is a Control Spell.
    pub fn get(&self, name: &str) -> Option<(Spell<'source>, bool)> {
        self.0.get_spell(name)
    }

    /// Iterates over all spells by their name.
    pub fn iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        self.0.iter_spells()
    }
}
