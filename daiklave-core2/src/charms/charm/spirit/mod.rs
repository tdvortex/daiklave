mod eclipse;
mod inner;
mod noneclipse;
mod id;
mod keyword;

pub use id::SpiritCharmId;
pub use keyword::SpiritCharmKeyword;

pub use eclipse::EclipseCharm;
pub use noneclipse::_NonEclipseCharm;

pub enum _SpiritCharm {
    Eclipse(EclipseCharm),
    NonEclipse(_NonEclipseCharm)
}