use crate::{
    artifact::MagicMaterial,
    book_reference::BookReference,
    weapons::{
        artifact::{
            BaseArtifactWeapon, NamedArtifactWeapon, NaturalArtifactWeapon,
            OneHandedArtifactWeapon, TwoHandedArtifactWeapon, WornArtifactWeapon,
        },
        ArtifactWeapon, BaseWeaponId,
    },
};

pub struct ArtifactWeaponBuilder<'build> {
    name: &'build str,
}

impl<'build> ArtifactWeaponBuilder<'build> {
    pub fn base_artifact(
        self,
        base_weapon_id: BaseWeaponId,
        base_weapon: BaseArtifactWeaponInsert,
    ) -> ArtifactWeaponBuilderWithBaseWeapon<'build> {
        ArtifactWeaponBuilderWithBaseWeapon {
            name: self.name,
            base_weapon_id,
            base_weapon,
        }
    }
}

pub struct ArtifactWeaponBuilderWithBaseWeapon<'build> {
    name: &'build str,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeaponInsert<'build>,
}

impl<'build> ArtifactWeaponBuilderWithBaseWeapon<'build> {
    pub fn material(
        self,
        magic_material: MagicMaterial,
    ) -> ArtifactWeaponBuilderWithMagicMaterial<'build> {
        ArtifactWeaponBuilderWithMagicMaterial {
            name: self.name,
            base_weapon_id: self.base_weapon_id,
            base_weapon: self.base_weapon,
            magic_material,
        }
    }
}

pub struct ArtifactWeaponBuilderWithMagicMaterial<'build> {
    name: &'build str,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeaponInsert<'build>,
    magic_material: MagicMaterial,
}

impl<'build> ArtifactWeaponBuilderWithMagicMaterial<'build> {
    pub fn merit_dots(self, dots: u8) -> ArtifactWeaponBuilderWithMeritDots<'build> {
        ArtifactWeaponBuilderWithMeritDots {
            name: self.name,
            base_weapon_id: self.base_weapon_id,
            base_weapon: self.base_weapon,
            magic_material: self.magic_material,
            merit_dots: dots,
        }
    }
}

pub struct ArtifactWeaponBuilderWithMeritDots<'build> {
    name: &'build str,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeaponInsert<'build>,
    magic_material: MagicMaterial,
    merit_dots: u8,
}

impl<'build> ArtifactWeaponBuilderWithMeritDots<'build> {
    pub fn hearthstone_slots(
        self,
        slots: usize,
    ) -> ArtifactWeaponBuilderWithHearthstoneSlots<'build> {
        ArtifactWeaponBuilderWithHearthstoneSlots {
            name: self.name,
            base_weapon_id: self.base_weapon_id,
            base_weapon: self.base_weapon,
            magic_material: self.magic_material,
            merit_dots: self.merit_dots,
            hearthstone_slots: slots,
            lore: None,
            powers: None,
            book_reference: None,
        }
    }
}

pub struct ArtifactWeaponBuilderWithHearthstoneSlots<'build> {
    name: &'build str,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeaponInsert<'build>,
    magic_material: MagicMaterial,
    merit_dots: u8,
    hearthstone_slots: usize,
    lore: Option<&'build str>,
    powers: Option<&'build str>,
    book_reference: Option<BookReference>,
}

impl<'view, 'build> ArtifactWeaponBuilderWithHearthstoneSlots<'build> {
    pub fn lore(mut self, lore: &'build str) -> Self {
        self.lore = Some(lore);
        self
    }

    pub fn powers(mut self, powers: &'build str) -> Self {
        self.powers = Some(powers);
        self
    }

    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn build(self) -> ArtifactWeapon<'build> {
        let (handedness, base_weapon) = match self.base_weapon {
            BaseArtifactWeaponInsert::Natural(base_weapon) => {
                (WeaponHandedness::Natural, base_weapon)
            }
            BaseArtifactWeaponInsert::Worn(base_weapon) => (WeaponHandedness::Worn, base_weapon),
            BaseArtifactWeaponInsert::OneHanded(base_weapon) => {
                (WeaponHandedness::OneHanded, base_weapon)
            }
            BaseArtifactWeaponInsert::TwoHanded(base_weapon) => {
                (WeaponHandedness::TwoHanded, base_weapon)
            }
        };

        let empty_hearthstone_slots = (0..self.hearthstone_slots).fold(Vec::new(), |mut v, _| {
            v.push(None);
            v
        });

        let named_artifact_weapon = NamedArtifactWeapon {
            name: self.name,
            book_reference: self.book_reference,
            merit_dots: self.merit_dots,
            base_weapon_id: self.base_weapon_id,
            base_weapon,
            lore: self.lore,
            powers: self.powers,
            hearthstone_slots: empty_hearthstone_slots,
        };

        match handedness {
            WeaponHandedness::Natural => {
                ArtifactWeapon::Natural(NaturalArtifactWeapon(named_artifact_weapon))
            }
            WeaponHandedness::Worn => {
                ArtifactWeapon::Worn(WornArtifactWeapon(named_artifact_weapon), false)
            }
            WeaponHandedness::OneHanded => {
                ArtifactWeapon::OneHanded(OneHandedArtifactWeapon(named_artifact_weapon), None)
            }
            WeaponHandedness::TwoHanded => {
                ArtifactWeapon::TwoHanded(TwoHandedArtifactWeapon(named_artifact_weapon), false)
            }
        }
    }
}

pub enum BaseArtifactWeaponInsert<'build> {
    Natural(BaseArtifactWeapon<'build>),
    Worn(BaseArtifactWeapon<'build>),
    OneHanded(BaseArtifactWeapon<'build>),
    TwoHanded(BaseArtifactWeapon<'build>),
}

enum WeaponHandedness {
    Natural,
    Worn,
    OneHanded,
    TwoHanded,
}
