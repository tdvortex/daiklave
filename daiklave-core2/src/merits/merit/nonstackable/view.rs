use super::with_dots::NonStackableMeritWithDots;

pub(crate) struct NonStackableMeritView<'source>(NonStackableMeritWithDots<'source>);

impl<'source> NonStackableMeritView<'source>{
    pub fn template_name(&self) -> &'source str {
        self.0.template_name()
    }
}