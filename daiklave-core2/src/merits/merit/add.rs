use crate::{artifact::AddArtifact, hearthstones::hearthstone::{AddHearthstone, AddManse}, sorcery::AddSorcery};

pub enum AddMerit {
    Artifact(AddArtifact),
    Demense(AddDemense),
    ExaltedHealing,
    Hearthstone(AddHearthstone),
    Languages(AddLanguages),
    Manse(AddManse),
    MartialArtist(AddMartialArtsStyle),
    MortalSorcery(AddSorcery),
    NonStackable(AddNonStackableMerit),
    SorceryArchetypeMerit(AddSorceryArchetypeMerit),
    Stackable(AddStackableMerit),
}