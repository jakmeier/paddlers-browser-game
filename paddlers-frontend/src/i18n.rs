pub type TextDb = gettext::Catalog;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextKey(&'static str);

impl From<&'static str> for TextKey {
    fn from(s: &'static str) -> Self {
        Self(s)
    }
}

impl TextKey {
    #[inline(always)]
    pub fn key(&self) -> &'static str {
        self.0
    }
}
