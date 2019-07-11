mod db;
mod town_defence;
mod attack_spawn;
mod resource_system;

use db::*;
use db_lib::sql::GameDB;

use std::time::Duration;
use std::thread;

fn main() {

    let db = DB::new();
    let mut t : u8 = 0;

    db.init_resources();

    loop {
        check_attacks(&db);
        if t == 0 {
            db.spawn_random_attack();
        }
        thread::sleep(Duration::from_millis(100));
        t = t.wrapping_add(1);
    }

}

fn check_attacks(db: &DB) {
    let attacks = db.attacks();
    let now = chrono::Local::now().naive_local();
    for atk in attacks.iter() {
        if atk.arrival < now {
            db.maybe_attack_now(atk, now -  atk.arrival);
        }
    }
}