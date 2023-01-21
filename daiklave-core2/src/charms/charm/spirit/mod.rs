mod eclipse;
mod id;
mod inner;
mod keyword;
mod noneclipse;

pub use id::SpiritCharmId;
pub use keyword::SpiritCharmKeyword;

pub use eclipse::EclipseCharm;
pub use noneclipse::_NonEclipseCharm;

pub enum _SpiritCharm {
    Eclipse(EclipseCharm),
    NonEclipse(_NonEclipseCharm),
}
