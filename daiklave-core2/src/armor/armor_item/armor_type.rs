use crate::{
    artifact::ArtifactId,
    book_reference::BookReference,
    hearthstones::{hearthstone::Hearthstone, HearthstonePosition},
};

use super::{
    artifact::{ArtifactArmorId, ArtifactArmorNoAttunement},
    mundane::MundaneArmorView,
    ArmorId, ArmorTag, ArmorWeightClass, BaseArmorId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ArmorType<'source> {
    Artifact(
        ArtifactArmorId,
        ArtifactArmorNoAttunement<'source>,
        Option<u8>,
    ),
    Mundane(BaseArmorId, MundaneArmorView<'source>),
}

impl<'source> ArmorType<'source> {
    pub fn id(&self) -> ArmorId {
        match self {
            ArmorType::Artifact(artifact_id, _, _) => ArmorId::Artifact(*artifact_id),
            ArmorType::Mundane(base_id, _) => ArmorId::Mundane(*base_id),
        }
    }

    pub fn name(&self) -> &'source str {
        match self {
            ArmorType::Artifact(_, artifact, _) => artifact.name(),
            ArmorType::Mundane(_, mundane) => mundane.name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            ArmorType::Artifact(_, artifact, _) => artifact.book_reference(),
            ArmorType::Mundane(_, mundane) => mundane.book_reference(),
        }
    }

    pub fn weight_class(&self) -> ArmorWeightClass {
        match self {
            ArmorType::Artifact(_, artifact, _) => artifact.base_armor().weight_class(),
            ArmorType::Mundane(_, mundane) => mundane.weight_class(),
        }
    }

    pub fn soak_bonus(&self) -> u8 {
        let weight_class = self.weight_class();
        let is_artifact = matches!(self, ArmorType::Artifact(..));

        match (weight_class, is_artifact) {
            (ArmorWeightClass::Light, false) => 3,
            (ArmorWeightClass::Medium, false) => 5,
            (ArmorWeightClass::Heavy, false) => 7,
            (ArmorWeightClass::Light, true) => 5,
            (ArmorWeightClass::Heavy, true) => 8,
            (ArmorWeightClass::Medium, true) => 11,
        }
    }

    pub fn mobility_penalty(&self) -> i8 {
        match self.weight_class() {
            ArmorWeightClass::Light => 0,
            ArmorWeightClass::Medium => -1,
            ArmorWeightClass::Heavy => -2,
        }
    }

    pub fn hardness(&self) -> u8 {
        match self {
            ArmorType::Artifact(_, _, _) => match self.weight_class() {
                ArmorWeightClass::Light => 4,
                ArmorWeightClass::Medium => 7,
                ArmorWeightClass::Heavy => 10,
            },
            ArmorType::Mundane(_, _) => 0,
        }
    }

    pub fn attunement_cost(&self) -> Option<u8> {
        match self {
            ArmorType::Artifact(_, _, _) => Some(match self.weight_class() {
                ArmorWeightClass::Light => 4,
                ArmorWeightClass::Medium => 5,
                ArmorWeightClass::Heavy => 6,
            }),
            ArmorType::Mundane(_, _) => None,
        }
    }

    pub fn is_attuned(&self) -> bool {
        match self {
            ArmorType::Artifact(_, _, attunement) => attunement.is_some(),
            ArmorType::Mundane(_, _) => false,
        }
    }

    pub fn tags(&self) -> impl Iterator<Item = ArmorTag> + '_ {
        match self {
            ArmorType::Artifact(_, artifact, _) => artifact.base_armor().tags(),
            ArmorType::Mundane(_, mundane) => mundane.tags(),
        }
    }

    pub fn hearthstone_slots(&self) -> u8 {
        match self {
            ArmorType::Artifact(_, artifact, _) => artifact.hearthstone_slots(),
            ArmorType::Mundane(_, _) => 0,
        }
    }

    pub fn slotted_hearthstones(&self) -> impl Iterator<Item = Hearthstone<'source>> {
        match self {
            ArmorType::Artifact(artifact_id, no_attunement, _) => no_attunement
                .slotted_hearthstones()
                .map(|slotted| {
                    Hearthstone(HearthstonePosition::Slotted(
                        ArtifactId::Armor(*artifact_id),
                        slotted,
                    ))
                })
                .collect(),
            ArmorType::Mundane(_, _) => vec![],
        }
        .into_iter()
    }

    pub fn open_slots(&self) -> u8 {
        match self {
            ArmorType::Artifact(_, no_attunement, _) => no_attunement.open_slots(),
            ArmorType::Mundane(_, _) => 0,
        }
    }
}
