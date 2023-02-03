mod add;
/// A builder path to construct a Spirit Charm.
pub mod builder;
mod eclipse;
mod inner;
mod keyword;
mod name;
mod spirit_only;

use std::collections::{HashMap, HashSet};

pub use add::AddSpiritCharm;
pub use keyword::SpiritCharmKeyword;

pub use eclipse::{AddEclipseCharm, EclipseCharm};
pub use name::SpiritCharmName;
pub use spirit_only::{AddSpiritOnlyCharm, SpiritOnlyCharm};

use self::builder::SpiritCharmBuilder;

/// A Charm which can be used by a Spirit.
pub enum SpiritCharm {
    /// The Charm is also learnable by Eclipse Caste Solars.
    Eclipse(EclipseCharm),
    /// The Charm is only usable by Spirits.
    SpiritOnly(SpiritOnlyCharm),
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
