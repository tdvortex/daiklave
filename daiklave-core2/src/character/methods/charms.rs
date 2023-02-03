use crate::{
    charms::{
        charm::{AddCharm, CharmName},
        Charms,
    },
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// Read the Charms (and Evocations and Spells) owned by the character.
    pub fn charms(&'view self) -> Charms<'view, 'source> {
        Charms(self)
    }

    /// Adds a Charm (or Spell) to the character.
    pub fn add_charm(
        &mut self,
        add_charm: &'source AddCharm,
    ) -> Result<&mut Self, CharacterMutationError> {
        match add_charm {
            AddCharm::Eclipse(add_eclipse_charm) => self.add_eclipse_charm(add_eclipse_charm),
            AddCharm::Evocation(add_evocation) => self.add_evocation(add_evocation),
            AddCharm::MartialArts(add_martial_arts_charm) => self.add_martial_arts_charm(add_martial_arts_charm),
            AddCharm::Solar(add_solar_charm) => self.add_solar_charm(add_solar_charm),
            AddCharm::Spell(add_spell) => self.add_spell(add_spell),
        }
    }

    /// Removes a Charm (or Spell) from the character.
    pub fn remove_charm(
        &mut self,
        remove_charm: CharmName<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        match remove_charm {
            CharmName::Spirit(name) => self.remove_spirit_charm(name),
            CharmName::Evocation(name) => self.remove_evocation(name),
            CharmName::MartialArts(name) => self.remove_martial_arts_charm(name),
            CharmName::Solar(name) => self.remove_solar_charm(name),
            CharmName::Spell(name) => self.remove_spell(name),
        }
    }
}
