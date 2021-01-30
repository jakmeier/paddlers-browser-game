use crate::prelude::BuildingType;

#[derive(Copy, Clone)]
/// Defines where in the frontend a dialogue scene is entered
pub enum DialogueEntry {
    /// Open scene immediately on load
    OnLoad,
    /// Entity trigger on Hero
    Hero,
    /// Entity trigger on Building of specific type
    Building(BuildingType),
}
