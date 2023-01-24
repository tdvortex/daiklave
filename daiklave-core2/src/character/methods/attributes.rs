use crate::{
    abilities::AbilityNameVanilla,
    attributes::{AttributeName, Attributes},
    weapons::weapon::{AttackRange, Equipped, WeaponWeightClass},
    Character, CharacterMutationError,
};

impl<'source> Character<'source> {
    /// Gets a struct reference for the character's attributes.
    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Sets the specified attribute name to the specified dot rating.
    pub fn set_attribute(
        &mut self,
        attribute_name: AttributeName,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        let old_dots = self.attributes().get(attribute_name).dots();
        self.attributes.set_dots(attribute_name, dots)?;
        if old_dots > dots {
            let sorcery_removed = if attribute_name == AttributeName::Intelligence {
                self.exaltation.correct_sorcery_level(
                    self.abilities().get(AbilityNameVanilla::Occult).dots(),
                    dots,
                    self.essence().map_or(1, |essence| essence.rating()),
                )
            } else {
                false
            };

            if attribute_name == AttributeName::Strength && dots < 3 {
                let maybe_weapon_id = self.weapons().iter().find_map(|(weapon_id, equipped)| {
                    if let Some(Equipped::TwoHanded) = equipped {
                        self.weapons().get(weapon_id, equipped).and_then(|weapon| {
                            if weapon.weight_class() == WeaponWeightClass::Heavy
                                && weapon.damage(AttackRange::Melee).is_some()
                            {
                                Some(weapon.id())
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    }
                });

                if let Some(weapon_id) = maybe_weapon_id {
                    self.unequip_weapon(weapon_id, Equipped::TwoHanded)?;
                }
            }

            self.correct_merits();
            // Evocations don't depend on attributes, but they may depend
            // on Spells
            if sorcery_removed {
                self.correct_evocations(&[]);
            }
        }
        Ok(self)
    }
}
