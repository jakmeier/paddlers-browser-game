//! This module has been auto-generate using specification loader.
#[derive(Clone,Copy,Debug)]
pub enum Quest {
    HelloWorld,
    CreateForest,
    BuildBundligStation,
    UseBundligStation,
}
impl Quest {
    pub fn key(&self) -> &'static str {
        match self {
            Self::HelloWorld => "hello-world",
            Self::CreateForest => "create-forest",
            Self::BuildBundligStation => "build-bundlig-station",
            Self::UseBundligStation => "use-bundlig-station",
        }
    }
}
