use crate::net::graphql::query_types::PlayerQueryResponse;

#[derive(Default, Debug, Clone, Copy)]
pub struct PlayerInfo {
    pub karma: i64,
}

impl From<PlayerQueryResponse> for PlayerInfo {
    fn from(p: PlayerQueryResponse) -> Self {
        PlayerInfo {
            karma: p.karma,
        }
    }
}