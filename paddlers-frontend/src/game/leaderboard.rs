use quicksilver::geom::Rectangle;
use crate::prelude::*;
use crate::gui::ui_state::UiState;
use crate::gui::input::UiView;
use stdweb::web::*;
// use stdweb::unstable::TryInto;

pub struct Leaderboard(Node);

impl UiState {
    pub fn init_leaderboard(&mut self, area: &Rectangle) -> PadlResult<Leaderboard> {
        let pane = panes::new_styled_pane(
            area.x() as u32,
            area.y() as u32,
            area.width() as u32,
            area.height() as u32,
            r#"<section class="leaderboard"></section>"#,
            &[""],
            &[("color","white")],
        )?;
        let node = pane.first_inner_node()?;

        // TODO Debug why this didn't work:
        
        // js! {
        //     console.log(@{node.as_ref()})
        // }

        // let el : HtmlElement = node.clone().try_into().map_err(
        //     |_| PadlError::dev_err(PadlErrorCode::InvalidDom("Not html"))
        // )?;

        // el.append_html(
        //     &format!(r#"<h3>{}</h3>
        //     <h3>{}</h3>
        //     <h3>{}</h3>
        //     "#,
        //     "#", "Name", "Karma")
        // ).expect("append html");

        // insert_h3(&node, "#");
        // insert_h3(&node, "Name");
        // insert_h3(&node, "Karma");
       
        pane.hide()?;
        self.view_panes.push((UiView::Leaderboard, pane));
        
        Ok(Leaderboard(node))
    }
}

// fn insert_h3(node: &Node, text: &str) {
//     let inner = document().create_element("h3").unwrap();
//     inner.set_text_content(text);
//     node.append_child(&inner);
//     std::mem::drop(inner);
// }

impl Leaderboard {
    pub fn clear(&self) -> PadlResult<()> {
        for node in self.0.child_nodes() {
            self.0.remove_child(&node).expect("not found");
        }
        Ok(())
    }
    
    pub fn insert_row(&self, rank: usize, name: &str, karma: i64) -> PadlResult<()> {
        
        let node = document().create_element("div").unwrap();
        node.set_text_content(&rank.to_string());
        self.0.append_child(&node);
        
        let node = document().create_element("div").unwrap();
        node.set_text_content(name);
        self.0.append_child(&node);
        
        let node = document().create_element("div").unwrap();
        node.set_text_content(&karma.to_string());
        self.0.append_child(&node);
        
        Ok(())
    }
}