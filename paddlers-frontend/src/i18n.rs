use serde::Deserialize;

pub type TextDb = gettext::Catalog;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextKey(&'static str);

#[derive(Deserialize)]
pub struct OwnedTextKey(String);

impl From<&'static str> for TextKey {
    fn from(s: &'static str) -> Self {
        Self(s)
    }
}
impl From<&str> for OwnedTextKey {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl TextKey {
    #[inline(always)]
    pub fn key(&self) -> &'static str {
        self.0
    }
}
impl OwnedTextKey {
    #[inline(always)]
    pub fn key(&self) -> &str {
        &self.0
    }
}
