//! This module has been auto-generate using specification loader.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuestName {
    HelloWorld,
    CreateForest,
    BuildBundligStation,
    UseBundligStation,
}
impl QuestName {
    pub fn unique_string(&self) -> &'static str {
        match self {
            Self::HelloWorld => "hello-world",
            Self::CreateForest => "create-forest",
            Self::BuildBundligStation => "build-bundlig-station",
            Self::UseBundligStation => "use-bundlig-station",
        }
    }
}

// Pseudo const-trait
impl QuestName {
    pub const fn const_eq(self, other: Self) -> bool {
        self as usize == other as usize
    }
}
