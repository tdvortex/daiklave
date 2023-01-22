use crate::exaltation::ExaltationSorcery;

use super::{spell::{Spell, SpellId}};

/// The spells known by a sorcerer.
pub struct Spells<'view, 'source>(pub(crate) ExaltationSorcery<'view, 'source>);

impl<'view, 'source> Spells<'view, 'source> {
    /// Gets a specific spell by its Id. The second parameter indicates if it
    /// is a Control Spell.
    pub fn get(&self, spell_id: SpellId) -> Option<(Spell<'source>, bool)> {
        self.0.get_spell(spell_id)
    }

    /// Iterates over all spells by their Id.
    pub fn iter(&self) -> impl Iterator<Item = SpellId> + '_ {
        self.0.iter_spells()
    }
}