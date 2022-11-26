use crate::abilities::Abilities;
use crate::attributes::Attributes;
use crate::merits::{Merit, MeritType, Merits};
use crate::weapons::{Hand, HasWeapons, WeaponDetails, WeaponPosition, Weapons, WeaponsIter};
use crate::willpower::{HasWillpower, Willpower};
use eyre::{eyre, Result};

#[derive(Default, Debug)]
pub struct MortalCharacter {
    pub attributes: Attributes,
    pub abilities: Abilities,
    pub merits: Merits,
    weapons: Weapons,
    willpower: Willpower,
}

impl HasWillpower for MortalCharacter {
    fn recover_one_willpower(&mut self) {
        self.willpower.recover_one()
    }

    fn recover_all_willpower(&mut self) {
        self.willpower.recover_all()
    }

    fn gain_one_willpower(&mut self) {
        self.willpower.gain_one()
    }

    fn spend_one_willpower(&mut self) -> Result<()> {
        self.willpower.spend_one()
    }
}

impl HasWeapons for MortalCharacter {
    fn weapons_iter(&self) -> WeaponsIter {
        self.weapons.iter()
    }

    fn get_weapon_at_position(&self, position: WeaponPosition) -> Result<&WeaponDetails> {
        self.weapons.get_at_position(position)
    }

    fn add_weapon(&mut self, weapon: WeaponDetails, two_handed: bool) {
        self.weapons.add_weapon(weapon, two_handed)
    }

    fn remove_weapon(&mut self, position: WeaponPosition) -> Result<()> {
        self.weapons.remove_weapon(position)
    }

    fn equip_weapon(&mut self, hand: Hand, position: WeaponPosition) -> Result<()> {
        self.weapons.equip_weapon(hand, position)
    }

    fn unequip_weapon(&mut self, hand: Hand) -> Result<()> {
        self.weapons.unequip_weapon(hand)
    }
}
