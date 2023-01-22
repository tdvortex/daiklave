/// A builder path to construct a Spirit Charm.
pub mod builder;
mod eclipse;
mod id;
mod inner;
mod keyword;
mod noneclipse;

use std::collections::{HashMap, HashSet};

pub use id::SpiritCharmId;
pub use keyword::SpiritCharmKeyword;

pub use eclipse::EclipseCharm;
pub use noneclipse::NonEclipseCharm;

use self::builder::SpiritCharmBuilder;

/// A Charm which can be used by a Spirit.
pub enum SpiritCharm {
    /// The Charm is also learnable by Eclipse Caste Solars.
    Eclipse(EclipseCharm),
    /// The Charm is only usable by Spirits.
    NonEclipse(NonEclipseCharm),
}

impl SpiritCharm {
    /// Starts a builder path to construct a new Spirit Charm.
    pub fn builder(name: String) -> SpiritCharmBuilder {
        SpiritCharmBuilder {
            name,
            book_reference: None,
            summary: None,
            keywords: HashSet::new(),
            costs: HashMap::new(),
        }
    }
}
