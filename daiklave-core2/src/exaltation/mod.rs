/// Traits common to all Exalts
pub mod exalt;

/// Traits which are unique to mortals (or which function differently without
///  Essence)
pub mod mortal;

mod exaltation;
mod exaltation_view;
mod martial_arts;
mod sorcery;

pub(crate) use exaltation::Exaltation;
pub(crate) use exaltation_view::ExaltationView;
pub(crate) use martial_arts::{ExaltationMartialArtist, ExaltationMartialArtistView};
pub(crate) use sorcery::{SorcerySwitch, SorceryViewSwitch};