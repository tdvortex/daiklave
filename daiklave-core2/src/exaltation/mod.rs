/// Traits common to all Exalts
pub mod exalt;

/// Traits which are unique to mortals (or which function differently without
///  Essence)
pub mod mortal;

mod exaltation;
mod exaltation_view;
pub(crate) mod martial_arts;

pub(crate) use exaltation::Exaltation;
pub(crate) use exaltation_view::ExaltationView;