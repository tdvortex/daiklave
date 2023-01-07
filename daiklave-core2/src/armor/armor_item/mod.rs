/// Properties of artifact armor
pub mod artifact;
mod base;
mod id;
mod memo;
/// Properties of mundane armor
pub mod mundane;

pub use id::ArmorId;
pub use base::BaseArmorId;

use self::base::builder::BaseArmorItemBuilder;

/// A single piece of armor owned by a character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArmorItem<'source> {
    name: &'source str,
}

impl<'source> ArmorItem<'source> {
    /// Starts constructing a base armor item.
    pub fn base(name: &str) -> BaseArmorItemBuilder {
        BaseArmorItemBuilder {
            name: name.to_owned()
        }
    }
}