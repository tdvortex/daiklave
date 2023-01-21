pub mod builder;
mod eclipse;
mod id;
mod inner;
mod keyword;
mod noneclipse;

use std::collections::{HashSet, HashMap};

pub use id::SpiritCharmId;
pub use keyword::SpiritCharmKeyword;

pub use eclipse::EclipseCharm;
pub use noneclipse::NonEclipseCharm;

use self::builder::SpiritCharmBuilder;

pub enum SpiritCharm {
    Eclipse(EclipseCharm),
    NonEclipse(NonEclipseCharm),
}

impl SpiritCharm {
    pub fn new(name: String) -> SpiritCharmBuilder {
        SpiritCharmBuilder {
            name,
            book_reference: None,
            summary: None,
            keywords: HashSet::new(),
            costs: HashMap::new(),
        }
    }
}
