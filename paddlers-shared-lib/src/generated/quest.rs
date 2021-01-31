//! This module has been auto-generate using specification loader.
#[derive(Clone,Copy,Debug)]
pub enum QuestName {
    HelloWorld,
    CreateForest,
    BuildBundlingStation,
    UseBundlingStation,
    Socialize,
    SocializeMore,
    BuildNest,
    GrowPopulation,
}
impl QuestName {
    pub fn unique_string(&self) -> &'static str {
        match self {
            Self::HelloWorld => "hello-world",
            Self::CreateForest => "create-forest",
            Self::BuildBundlingStation => "build-bundling-station",
            Self::UseBundlingStation => "use-bundling-station",
            Self::Socialize => "socialize",
            Self::SocializeMore => "socialize-more",
            Self::BuildNest => "build-nest",
            Self::GrowPopulation => "grow-population",
        }
    }
}
impl std::str::FromStr for QuestName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hello-world" => Ok(Self::HelloWorld),
            "create-forest" => Ok(Self::CreateForest),
            "build-bundling-station" => Ok(Self::BuildBundlingStation),
            "use-bundling-station" => Ok(Self::UseBundlingStation),
            "socialize" => Ok(Self::Socialize),
            "socialize-more" => Ok(Self::SocializeMore),
            "build-nest" => Ok(Self::BuildNest),
            "grow-population" => Ok(Self::GrowPopulation),
            _ => Err(()),
        }
    }
}
