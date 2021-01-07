use crate::api::keys::QuestKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QuestCollect {
    pub quest: QuestKey,
}
