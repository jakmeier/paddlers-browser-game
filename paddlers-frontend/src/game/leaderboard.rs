use crate::game::Game;
use crate::gui::utils::colors::DARK_BLUE;
use crate::net::NetMsg;
use crate::prelude::*;
use div::doc;
use paddle::Frame;
use paddle::NutsCheck;
use web_sys::Node;

pub(crate) struct LeaderboardFrame {
    pane: div::DivHandle,
    table: Node,
    players_by_karma: Vec<(String, i64)>,
    page_size: usize,
    current_page: usize,
}

impl LeaderboardFrame {
    pub fn new() -> PadlResult<Self> {
        let area = Self::area();
        let pane = div::new_styled(
            area.x() as u32,
            area.y() as u32,
            area.width() as u32,
            area.height() as u32,
            r#"<section class="leaderboard"></section>"#,
            &[""],
            &[("color", "white")],
        )?;
        let table_node = pane.first_inner_node()?;

        pane.hide()?;

        Ok(LeaderboardFrame {
            pane,
            table: table_node,
            players_by_karma: vec![],
            page_size: 15,
            current_page: 0,
        })
    }
    pub fn clear(&self) -> PadlResult<()> {
        while let Some(child) = self.table.last_child() {
            self.table.remove_child(&child)?;
        }
        Ok(())
    }

    pub fn insert_row(&self, rank: usize, name: &str, karma: i64) -> PadlResult<()> {
        let node = doc()?.create_element("div")?;
        node.set_text_content(Some(&rank.to_string()));
        self.table.append_child(&node)?;

        let node = doc()?.create_element("div")?;
        node.set_text_content(Some(&name));
        self.table.append_child(&node)?;

        let node = doc()?.create_element("div")?;
        node.set_text_content(Some(&karma.to_string()));
        self.table.append_child(&node)?;

        Ok(())
    }

    pub fn network_message(&mut self, _state: &mut Game, msg: &NetMsg) {
        match msg {
            NetMsg::Leaderboard(offset, list) => {
                let required_len = offset + list.len();
                if self.players_by_karma.len() < required_len {
                    self.players_by_karma
                        .resize(required_len, Default::default())
                }
                for (i, (name, karma)) in list.into_iter().enumerate() {
                    self.players_by_karma[offset + i] = (name.clone(), *karma);
                }
                if self.current_page * self.page_size < required_len
                    && (self.current_page + 1) * self.page_size > *offset
                {
                    self.reload();
                }
            }
            _ => {}
        }
    }
    fn reload(&mut self) {
        self.clear().nuts_check();
        let start = self.current_page * self.page_size;
        let end = start + self.page_size;
        for (i, (name, karma)) in self.players_by_karma[start..end].iter().enumerate() {
            self.insert_row(start + i + 1, &name, *karma).nuts_check();
        }
    }
}

impl Frame for LeaderboardFrame {
    type State = Game;
    const WIDTH: u32 = crate::resolution::MAIN_AREA_W;
    const HEIGHT: u32 = crate::resolution::MAIN_AREA_H;
    fn draw(
        &mut self,
        _state: &mut Self::State,
        window: &mut paddle::DisplayArea,
        _timestamp: f64,
    ) {
        window.fill(&DARK_BLUE);
    }
    fn enter(&mut self, _state: &mut Self::State) {
        crate::net::request_leaderboard(self.current_page as i64, self.page_size as i64);
        self.pane.show().nuts_check();
    }
    fn leave(&mut self, _state: &mut Self::State) {
        self.pane.hide().nuts_check();
    }
}
