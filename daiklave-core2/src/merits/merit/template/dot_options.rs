#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MeritTemplateDotOptions {
    Fixed(u8),
    Variable([Option<String>; 6]),
}
