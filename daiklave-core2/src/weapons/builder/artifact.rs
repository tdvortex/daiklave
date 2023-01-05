use crate::{book_reference::BookReference, artifact::MagicMaterial, weapons::{BaseWeaponId, ArtifactWeapon}};

pub struct ArtifactWeaponBuilder<'build> {
    name: &'build str
}

impl<'build> ArtifactWeaponBuilder<'build> {
    pub fn base_artifact(self, base_artifact_id: BaseWeaponId, base_artifact: BaseArtifactWeaponInsert) -> ArtifactWeaponBuilderWithBaseWeapon<'build> {
        todo!()
    }
}

pub struct ArtifactWeaponBuilderWithBaseWeapon<'build> {
    name: &'build str
}

impl<'build> ArtifactWeaponBuilderWithBaseWeapon<'build> {
    pub fn material(self, magic_material: MagicMaterial) -> ArtifactWeaponBuilderWithMagicMaterial<'build> {
        todo!()
    }
}

pub struct ArtifactWeaponBuilderWithMagicMaterial<'build> {
    name: &'build str
}

impl<'build> ArtifactWeaponBuilderWithMagicMaterial<'build> {
    pub fn merit_dots(self, dots: u8) -> ArtifactWeaponBuilderWithMeritDots<'build> {
        todo!()
    }

}

pub struct ArtifactWeaponBuilderWithMeritDots<'build> {
    name: &'build str
}

impl<'build> ArtifactWeaponBuilderWithMeritDots<'build> {
    pub fn hearthstone_slots(self, slots: usize) -> ArtifactWeaponBuilderWithHearthstoneSlots<'build> {
        todo!()
    }
}

pub struct ArtifactWeaponBuilderWithHearthstoneSlots<'build> {
    name: &'build str
}

impl<'view, 'build> ArtifactWeaponBuilderWithHearthstoneSlots<'build> {
    pub fn lore(self, lore: &'build str) -> Self {
        todo!()
    }

    pub fn powers(self, powers: &'build str) -> Self {
        todo!()
    }

    pub fn book_reference(self, book_reference: BookReference) -> Self {
        todo!()
    }

    pub fn build(&'view self) -> ArtifactWeapon<'view, 'build> {
        todo!()
    }
}

pub struct BaseArtifactWeaponInsert {}

