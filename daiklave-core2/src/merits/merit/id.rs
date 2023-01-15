use crate::{artifact::ArtifactId, hearthstones::HearthstoneId, martial_arts::MartialArtsStyleId, unique_id::UniqueId, languages::language::MajorLanguage};

use super::{stackable::StackableMeritId, nonstackable::NonStackableMeritId};

/// The Id for a specific instance of a merit as owned by a character. Not to
/// be confused with MeritTemplateId, which describes a specific merit without
/// any identifying details.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MeritId {
    /// The merit associated with a specific owned Artifact.
    Artifact(ArtifactId),
    /// The merit associated with a standalone Demense, without a manse.
    DemenseNoManse(UniqueId),
    /// The merit associated with a Demense acquired through the Manse merit.
    /// Keys off the hearthstone associated with the manse and demense.
    DemenseWithManse(HearthstoneId),
    /// The Exalted Healing merit, which all Exalts get for free but mortals
    /// must purchase as a Supernatural merit.
    ExaltedHealing,
    /// The merit associated with a specific Hearthstone which was acquired
    /// without the Manse merit.
    HearthstoneNoManse(HearthstoneId),
    /// The merit associated with a Hearthstone acquired for free with the
    /// Manse merit.
    HearthstoneWithManse(HearthstoneId),
    /// The Local Tongues variant of the Languages merit.
    LocalTongues,
    /// The Language Merit, except for its Local Tongues variant.
    MajorLanguage(MajorLanguage),
    /// The merit associated with the Manse merit. Keys off the hearthstone
    /// acquired with it.
    Manse(HearthstoneId),
    /// The merit associated with the acquisition of a specific Martial Arts
    /// style.
    MartialArtist(MartialArtsStyleId),
    /// For mortals, the merit associated with the acquisition of Terrestrial
    /// circle sorcery. 
    MortalSorcerer,
    /// A merit which is not stackable and requires no additional detail, apart
    /// from the number of dots invested. Examples include Ambidextrous and 
    /// Pain Tolerance.
    NonStackableMerit(NonStackableMeritId),
    /// A merit which is stackable and requires a qualifying descriptor for
    /// each purchase. Examples include Allies, Language, or Resources.
    StackableMerit(StackableMeritId),
}