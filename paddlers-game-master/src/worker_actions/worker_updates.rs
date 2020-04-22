use paddlers_shared_lib::game_mechanics::worker::hero_level_exp;
use paddlers_shared_lib::models::Worker;

pub trait MutWorkerDBEntity {
    fn add_exp(&mut self, n: i32);
}

impl MutWorkerDBEntity for Worker {
    fn add_exp(&mut self, n: i32) {
        self.exp += n;
        while self.exp >= hero_level_exp(self.level) {
            self.exp -= hero_level_exp(self.level);
            self.level += 1;
        }
    }
}
