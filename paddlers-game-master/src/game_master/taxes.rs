//! Taxes are end-of-the-day rewards provided for each hobo in a village

use super::{event::Event, event_queue::EventQueue};
use crate::db::DB;
use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use paddlers_shared_lib::{
    keys::SqlKey,
    prelude::{GameDB, NewVisitReport, Village},
};
use rand::Rng;

impl EventQueue {
    pub fn next_tax_collection() -> DateTime<Utc> {
        let now = chrono::Utc::now().naive_utc() + Duration::seconds(10);
        let tonight = now
            .with_hour(23)
            .unwrap()
            .with_minute(59)
            .unwrap()
            .with_second(59)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        Utc.from_utc_datetime(&tonight)
    }
    pub fn schedule_tax_collection(&mut self) {
        self.add_event(Event::PayTaxes, Self::next_tax_collection())
    }
}

impl DB {
    pub fn pay_taxes_to_all_players(&self) {
        println!("Collecting taxes");
        let mut rng = rand::thread_rng();
        let seed = rng.gen();
        for village in self.all_player_villages() {
            self.collect_taxes(village, seed);
        }
    }
    fn collect_taxes(&self, village: Village, seed: u8) {
        let hobos = self.hobos(village.key());
        for hobo in &hobos {
            if hobo.nest.is_some() {
                let mut report = NewVisitReport {
                    sender: Some(hobo.id),
                    village_id: village.id,
                    karma: 0,
                };
                let mut feathers = 0;
                let mut logs = 0;
                report.karma += 1;
                match (hobo.id.wrapping_mul(seed as i64).abs() + seed as i64) % 255 {
                    0 => feathers += 3,
                    1..20 => logs += 1,
                    11..60 => feathers += 1,
                    _ => {}
                }
                self.add_new_report(report, feathers, 0, logs);
            }
        }
    }
}
