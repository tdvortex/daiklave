/// Traits relating to specific Solar castes.
pub mod caste;

mod builder;
mod builder_error;
mod solar_memo;
mod solar_view;
mod sorcery;

pub use solar_view::SolarView;

pub use solar_memo::SolarMemo;
pub(crate) use sorcery::{SolarSorcererMemo, SolarSorcererView};
