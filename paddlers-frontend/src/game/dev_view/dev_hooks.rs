use crate::game::*;
use crate::prelude::ScreenResolution;
use paddle::JmrRectangle;
use stdweb::unstable::TryInto;

impl Game {
    #[cfg(feature = "dev_view")]
    pub fn start_update(&mut self) {
        if let Some(test) = self.active_test.as_mut() {
            test.record_start_of_update();
        }
    }
    #[cfg(feature = "dev_view")]
    pub fn end_update(&mut self) {
        if let Some(test) = self.active_test.as_mut() {
            test.record_end_of_update();
        }
    }
    #[cfg(feature = "dev_view")]
    pub fn start_draw(&mut self) {
        if let Some(test) = self.active_test.as_mut() {
            test.record_start_of_frame();
        }
    }
    #[cfg(feature = "dev_view")]
    pub fn end_draw(&mut self) {
        if let Some(test) = self.active_test.as_mut() {
            test.record_end_of_frame();
            if let Some(result) = test.result() {
                let date = utc_now().timestamp();
                #[cfg(feature = "mobile_debug")]
                let device = "phone";
                #[cfg(not(feature = "mobile_debug"))]
                let device = "laptop";
                let version = env!("CARGO_PKG_VERSION");
                let navigator = web_sys::window().unwrap().navigator();
                let user_agent = navigator
                    .user_agent()
                    .unwrap_or_else(|| "NotAvailable".to_owned());
                let resolution = *self.world.fetch::<ScreenResolution>();
                println!(
                    "{} {} {:?} {} \"{}\" {:?} {}",
                    date, version, test.kind, device, user_agent, resolution, result
                );
                self.active_test = None;
            }
        }
    }
    #[cfg(feature = "dev_view")]
    pub fn draw_dev_view(&mut self, window: &mut Window) {
        if self.palette {
            let area =
                Rectangle::new((0, 0), window.project() * window.browser_region().size()).padded(100.0);
            crate::gui::utils::colors::palette::draw_color_palette(window, area);
        }
    }
    #[cfg(feature = "dev_view")]
    pub fn dev_view_event(&mut self, event: &Event) {
        match event {
            Event::Key(key, state) if *key == Key::Space && *state == ButtonState::Pressed => {
                self.palette = !self.palette;
            }
            Event::Key(key, state)
                if *state == ButtonState::Pressed && self.active_test.is_none() =>
            {
                match key {
                    Key::T => {
                        let test = dev_view::benchmark::Test::Vanilla;
                        self.active_test = Some(Box::new(
                            crate::game::dev_view::benchmark::TestData::start_test(self, test),
                        ));
                    }
                    Key::Key1 => {
                        let test = dev_view::benchmark::Test::Empty;
                        self.active_test = Some(Box::new(
                            crate::game::dev_view::benchmark::TestData::start_test(self, test),
                        ));
                    }
                    Key::Key2 => {
                        let test = dev_view::benchmark::Test::StandardVillage;
                        self.active_test = Some(Box::new(
                            crate::game::dev_view::benchmark::TestData::start_test(self, test),
                        ));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
