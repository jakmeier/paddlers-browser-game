use quicksilver::geom::Rectangle;
use super::FloatingText;

/// Allocates FloatingText units
pub struct TextPool {
    pool: Vec<FloatingText>,
    used: usize,
    factory_html: String,
    factory_css: Vec<(&'static str,&'static str)>,
    factory_pos: Rectangle,
}

impl TextPool {
    pub fn new(html: String, styles: &[(&'static str,&'static str)], pos: Rectangle) -> Self {
        let mut css = vec![];
        css.extend_from_slice(styles);
        TextPool {
            factory_html: html,
            factory_css: css,
            factory_pos: pos,
            pool: vec![],
            used: 0,
        }
    }
    pub fn allocate(&mut self) -> &mut FloatingText {
        if self.pool.len() <= self.used {
            self.increase_pool();
        }
        let i = self.used;
        self.used += 1;
        self.pool[i].show().expect("float");
        &mut self.pool[i]
    }
    fn increase_pool(&mut self) {
        self.pool.push(
            FloatingText::new_styled(
                &self.factory_pos,
                self.factory_html.clone(),
                &self.factory_css,
            )
            .expect("FloatingText creation failed")
        );
    }
    pub fn reset(&mut self) {
        self.used = 0;
    }
    pub fn finish_draw(&mut self) {
        if self.used < self.pool.len() {
            for float in &self.pool[self.used..] {
                float.hide().expect("float");
            }
        }
    }
    pub fn hide(&mut self) {
        for float in &self.pool {
            float.hide().expect("float");
        }
    }
}
impl Default for TextPool {
    fn default() -> Self {
        TextPool {
            factory_html: "".to_owned(),
            factory_pos: Rectangle::default(),
            factory_css: vec![],
            pool: vec![],
            used: 0,
        }
    }
}