/// Traits relating to specific Solar castes.
pub mod caste;

mod builder;
mod builder_error;
mod sorcery;
mod solar_memo;
mod solar_view;


pub use solar_view::SolarView;

pub use solar_memo::SolarMemo;
pub(crate) use sorcery::{SolarSorcererMemo, SolarSorcererView};