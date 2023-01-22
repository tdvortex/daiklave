mod builder;
pub use builder::CharmBuilder;

/// Evocations of artifacts and hearthstones.
pub mod evocation;
mod id;
mod mutation;
/// Evocations of spirits (and other supernatural beings), as well as Eclipse
/// Charms.
pub mod spirit;
use crate::{
    exaltation::exalt::exalt_type::solar::charm::SolarCharm, martial_arts::charm::MartialArtsCharm,
    sorcery::spell::Spell,
};
pub use mutation::CharmMutation;

use self::evocation::Evocation;

pub use id::CharmId;
pub use spirit::{EclipseCharm, SpiritCharmId, SpiritCharmKeyword};

/// A Charm possessed by a character.
pub enum Charm<'source> {
    /// A Spirit charm with the Eclipse keyword, purchasable by Eclipse caste
    /// Solars.
    Eclipse(&'source EclipseCharm),
    /// An Evocation of an artifact or hearthstone.
    Evocation(&'source Evocation),
    /// A Martial Arts charm for a specific style.
    MartialArts(&'source MartialArtsCharm),
    /// A Solar charm.
    Solar(&'source SolarCharm),
    /// A Spell.
    Spell(Spell<'source>),
}

impl<'source> Charm<'source> {
    /// Begins construction of a new Charm.
    pub fn builder(name: String) -> CharmBuilder {
        CharmBuilder {
            name,
            book_reference: None,
            summary: None,
        }
    }
}
