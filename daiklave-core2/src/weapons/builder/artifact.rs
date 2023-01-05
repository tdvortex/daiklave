use crate::{
    artifact::MagicMaterial,
    book_reference::{BookReference},
    weapons::{
        artifact::{
            BaseArtifactWeapon, NamedArtifactWeapon, NaturalArtifactWeapon,
            OneHandedArtifactWeapon, TwoHandedArtifactWeapon, WornArtifactWeapon,
        },
        ArtifactWeapon, BaseWeaponId,
    },
};

/// A builder to construct a new artifact weapon. Enforces that required fields
/// are specified in order: name, base artifact, magic material, merit dots,
/// and finally hearthstone slots. Optional fields (lore, powers, and book 
/// reference) may be specified at any time prior to the final build().
pub struct ArtifactWeaponBuilder<'build> {
    name: &'build str,
    lore: Option<&'build str>,
    powers: Option<&'build str>,
    book_reference: Option<BookReference>,
}

impl<'build> ArtifactWeaponBuilder<'build> {
    /// Add flavor text to describe the weapon's forging, history, and prior
    /// wielders.
    pub fn lore(mut self, lore: &'build str) -> Self {
        self.lore = Some(lore);
        self
    }

    /// Add passive or unique magical effects that are not Evocations, such as
    /// Beloved Adorei's emotional bond to her wielder.
    pub fn powers(mut self, powers: &'build str) -> Self {
        self.powers = Some(powers);
        self
    }

    /// Add a book reference for the weapon. Note that this is a reference for
    /// the named instance of the artifact and not the base weapon.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Specifies the base artifact weapon for the artifact. 
    pub fn base_artifact(
        self,
        base_weapon_id: BaseWeaponId,
        base_weapon: BaseArtifactWeaponInsert<'build>,
    ) -> ArtifactWeaponBuilderWithBaseWeapon<'build> {
        ArtifactWeaponBuilderWithBaseWeapon {
            name: self.name,
            base_weapon_id,
            base_weapon,
            lore: self.lore,
            powers: self.powers,
            book_reference: self.book_reference,
        }
    }
}

/// An artifact builder after the base weapon has been specified.
/// The next stage is .material().
pub struct ArtifactWeaponBuilderWithBaseWeapon<'build> {
    name: &'build str,
    lore: Option<&'build str>,
    powers: Option<&'build str>,
    book_reference: Option<BookReference>,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeaponInsert<'build>,
}

impl<'build> ArtifactWeaponBuilderWithBaseWeapon<'build> {
    /// Add flavor text to describe the weapon's forging, history, and prior
    /// wielders.
    pub fn lore(mut self, lore: &'build str) -> Self {
        self.lore = Some(lore);
        self
    }

    /// Add passive or unique magical effects that are not Evocations, such as
    /// Beloved Adorei's emotional bond to her wielder.
    pub fn powers(mut self, powers: &'build str) -> Self {
        self.powers = Some(powers);
        self
    }

    /// Add a book reference for the weapon. Note that this is a reference for
    /// the named instance of the artifact and not the base weapon.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Specifies the magic material from which the weapon is constructed. If
    /// a weapon is built with more than one, only the primary material is 
    /// recorded and the accents can be listed under Lore.
    pub fn material(
        self,
        magic_material: MagicMaterial,
    ) -> ArtifactWeaponBuilderWithMagicMaterial<'build> {
        ArtifactWeaponBuilderWithMagicMaterial {
            name: self.name,
            base_weapon_id: self.base_weapon_id,
            base_weapon: self.base_weapon,
            magic_material,
            lore: self.lore,
            powers: self.powers,
            book_reference: self.book_reference,
        }
    }
}

/// An artifact weapon after specifying its Magic Material. The next
/// step is .merit_dots().
pub struct ArtifactWeaponBuilderWithMagicMaterial<'build> {
    name: &'build str,
    lore: Option<&'build str>,
    powers: Option<&'build str>,
    book_reference: Option<BookReference>,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeaponInsert<'build>,
    magic_material: MagicMaterial,
}

impl<'build> ArtifactWeaponBuilderWithMagicMaterial<'build> {
    /// Add flavor text to describe the weapon's forging, history, and prior
    /// wielders.
    pub fn lore(mut self, lore: &'build str) -> Self {
        self.lore = Some(lore);
        self
    }

    /// Add passive or unique magical effects that are not Evocations, such as
    /// Beloved Adorei's emotional bond to her wielder.
    pub fn powers(mut self, powers: &'build str) -> Self {
        self.powers = Some(powers);
        self
    }

    /// Add a book reference for the weapon. Note that this is a reference for
    /// the named instance of the artifact and not the base weapon.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Specifies the dot rating of the artifact. Officially, all artifact 
    /// weapons should be rated 3+, but this is not enforced. Dot ratings 
    /// of 6+ are treatedas N/A artifacts. 
    pub fn merit_dots(self, dots: u8) -> ArtifactWeaponBuilderWithMeritDots<'build> {
        ArtifactWeaponBuilderWithMeritDots {
            name: self.name,
            base_weapon_id: self.base_weapon_id,
            base_weapon: self.base_weapon,
            magic_material: self.magic_material,
            merit_dots: dots.min(6),
            lore: self.lore,
            powers: self.powers,
            book_reference: self.book_reference,
        }
    }
}

/// An artifact builder after the number of merit dots is specified.
/// The next step is .hearthstone_slots().
pub struct ArtifactWeaponBuilderWithMeritDots<'build> {
    name: &'build str,
    lore: Option<&'build str>,
    powers: Option<&'build str>,
    book_reference: Option<BookReference>,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeaponInsert<'build>,
    magic_material: MagicMaterial,
    merit_dots: u8,
}

impl<'build> ArtifactWeaponBuilderWithMeritDots<'build> {
    /// Add flavor text to describe the weapon's forging, history, and prior
    /// wielders.
    pub fn lore(mut self, lore: &'build str) -> Self {
        self.lore = Some(lore);
        self
    }

    /// Add passive or unique magical effects that are not Evocations, such as
    /// Beloved Adorei's emotional bond to her wielder.
    pub fn powers(mut self, powers: &'build str) -> Self {
        self.powers = Some(powers);
        self
    }

    /// Add a book reference for the weapon. Note that this is a reference for
    /// the named instance of the artifact and not the base weapon.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }
    
    /// Puts a number of (empty) hearthstone slots into the weapon.
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

/// An artifact builder after having its hearthstone slots specified.
/// The final step is .build() to finish the builder.
pub struct ArtifactWeaponBuilderWithHearthstoneSlots<'build> {
    name: &'build str,
    lore: Option<&'build str>,
    powers: Option<&'build str>,
    book_reference: Option<BookReference>,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeaponInsert<'build>,
    magic_material: MagicMaterial,
    merit_dots: u8,
    hearthstone_slots: usize,
}

impl<'view, 'build> ArtifactWeaponBuilderWithHearthstoneSlots<'build> {
    /// Add flavor text to describe the weapon's forging, history, and prior
    /// wielders.
    pub fn lore(mut self, lore: &'build str) -> Self {
        self.lore = Some(lore);
        self
    }

    /// Add passive or unique magical effects that are not Evocations, such as
    /// Beloved Adorei's emotional bond to her wielder.
    pub fn powers(mut self, powers: &'build str) -> Self {
        self.powers = Some(powers);
        self
    }

    /// Add a book reference for the weapon. Note that this is a reference for
    /// the named instance of the artifact and not the base weapon.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Completes the builder, returning an Artifact Weapon.
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
            magic_material: self.magic_material,
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

/// A base artifact weapon to be inserted into a character. This
/// wraps the BaseArtifactWeapon struct with its wielding characteristics.
pub enum BaseArtifactWeaponInsert<'build> {
    /// A Natural base artifact weapon (uncommon).
    Natural(BaseArtifactWeapon<'build>),
    /// A Worn base artifact weapon like Smashfists.
    Worn(BaseArtifactWeapon<'build>),
    /// A One-Handed base artifact weapon.
    OneHanded(BaseArtifactWeapon<'build>),
    /// A Two-Handed base artifact weapon.
    TwoHanded(BaseArtifactWeapon<'build>),
}

enum WeaponHandedness {
    Natural,
    Worn,
    OneHanded,
    TwoHanded,
}
