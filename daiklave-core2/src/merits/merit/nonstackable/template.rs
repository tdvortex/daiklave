use crate::merits::merit::template::MeritTemplate;

use super::NonStackableMeritId;

pub struct NonStackableMeritTemplate(pub(crate) NonStackableMeritId, pub(crate) MeritTemplate);