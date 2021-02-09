use serde::Deserialize;
#[derive(Deserialize)]
#[serde(transparent)]
pub struct OwnedTextKey(String);
impl From<&str> for OwnedTextKey {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}
impl OwnedTextKey {
    #[inline(always)]
    pub fn key(&self) -> &str {
        &self.0
    }
}
