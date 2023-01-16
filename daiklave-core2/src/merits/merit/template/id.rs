use crate::{
    merits::merit::{nonstackable::NonStackableMeritId, stackable::StackableMeritTemplateId},
    sorcery::SorceryArchetypeMeritId,
};

pub enum MeritTemplateId {
    /// The template for the Artifact merit.
    Artifact,
    /// The template associated with the Demense merit.
    Demense,
    /// The template associated with the Exalted Healing merit.
    ExaltedHealing,
    /// The template associated with the Hearthstone merit.
    Hearthstone,
    /// The template associate with the Language merit.
    Language,
    /// The template associated with the Manse merit.
    Manse,
    /// The template associated with the Martial Artist merit.
    MartialArtist,
    /// The template associated with the Mortal Sorcerer merit.
    MortalSorcerer,
    /// The template for a non-stackable merit, unique for each character. Note
    /// that the template Id is the same as the instance Id.
    NonStackable(NonStackableMeritId),
    /// The template for a sorcery archetype merit. These are always
    /// non-stackable.
    SorceryArchetype(SorceryArchetypeMeritId),
    /// The template for a stackable merit, which is not necessarily unique for
    /// a character. Note that the
    Stackable(StackableMeritTemplateId),
}
