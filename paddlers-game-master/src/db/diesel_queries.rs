use super::*;
use diesel::*;
use paddlers_shared_lib::{
    civilization::CivilizationPerk, civilization::CivilizationPerks, models::dsl, schema::*,
    story::story_state::StoryState,
};

impl DB {
    pub fn insert_player(&self, u: &NewPlayer) -> QueryResult<Player> {
        diesel::insert_into(players::dsl::players)
            .values(u)
            .get_result(self.dbconn())
    }
    pub fn set_story_state(&self, p: PlayerKey, story_state: StoryState) -> QueryResult<Player> {
        let target = players::table.find(p.num());
        diesel::update(target)
            .set(players::story_state.eq(story_state))
            .get_result(self.dbconn())
    }
    pub fn unlock_civ_perk(&self, p: PlayerKey, perk: CivilizationPerk) -> QueryResult<Player> {
        let target = players::table.find(p.num());
        let mut new_perks = CivilizationPerks::new(0);
        new_perks.set(perk);
        let mask = new_perks.encode() as i64;
        // This should be done with bitwise or (|) instead of an additional lookup. It seems not supported in diesel out-of-the-box. However, `diesel_infix_operator!` should offer a solution, but is it worth it? (I tried for ~30min and then gave up.)
        let old_value = self.player(p).unwrap().civ_perks;
        diesel::update(target)
            .set(players::civ_perks.eq(mask | old_value))
            .get_result(self.dbconn())
    }

    pub fn delete_attack_hobos(&self, atk: AttackKey) {
        // Performance: This is a lot of sequential queries, could be reduced to one
        for hobo in self.attack_hobos(atk) {
            let result = diesel::delete(&hobo).execute(self.dbconn());
            if result.is_err() {
                println!("Couldn't delete hobo {:?}", hobo);
            }
        }
    }
    /// Releases a queued attack from the watergate. The village filter can be used to ensure no foreign attacks a released.
    pub fn start_fight(&self, atk: AttackKey, village_filter: Option<VillageKey>) {
        let query = diesel::update(attacks::table)
            .filter(attacks::id.eq(atk.num()))
            .set(attacks::entered_destination.eq(diesel::dsl::now.at_time_zone("UTC").nullable()));
        let result;
        if let Some(VillageKey(vid)) = village_filter {
            result = query
                .filter(attacks::destination_village_id.eq(vid))
                .execute(self.dbconn())
        } else {
            result = query.execute(self.dbconn())
        };
        if result.is_err() {
            println!("Couldn't start fight {:?}", atk);
        }
    }
    pub fn delete_attack(&self, atk: &Attack) {
        let result = diesel::delete(atk).execute(self.dbconn());
        if result.is_err() {
            println!("Couldn't delete attack {:?}", atk);
        }
    }

    pub fn insert_hobo(&self, u: &NewHobo) -> Hobo {
        diesel::insert_into(hobos::dsl::hobos)
            .values(u)
            .get_result(self.dbconn())
            .expect("Inserting hobo")
    }
    pub fn insert_worker(&self, u: &NewWorker) -> Worker {
        diesel::insert_into(workers::dsl::workers)
            .values(u)
            .get_result(self.dbconn())
            .expect("Inserting worker")
    }
    pub fn update_worker(&self, u: &Worker) {
        diesel::update(u)
            .set(u)
            .execute(self.dbconn())
            .expect("Updating worker");
    }
    pub fn add_worker_mana(&self, w: WorkerKey, plus: i32, max: i32) {
        let worker = self.worker_priv(w).expect("Invalid worker key");
        diesel::update(&worker)
            .set(workers::mana.eq(Some(max.min(worker.mana.unwrap_or(0) + plus))))
            .execute(self.dbconn())
            .expect("Updating worker for mana");
    }
    pub fn insert_attack(&self, new_attack: &NewAttack) -> Attack {
        diesel::insert_into(attacks::dsl::attacks)
            .values(new_attack)
            .get_result(self.dbconn())
            .expect("Inserting attack")
    }
    pub fn insert_attack_to_hobo(&self, atu: &AttackToHobo) {
        diesel::insert_into(attacks_to_hobos::dsl::attacks_to_hobos)
            .values(atu)
            .execute(self.dbconn())
            .expect("Inserting attack to hobo");
    }
    pub fn insert_resource(&self, res: &Resource) -> QueryResult<usize> {
        diesel::insert_into(dsl::resources)
            .values(res)
            .execute(self.dbconn())
    }
    pub fn add_resource(
        &self,
        rt: ResourceType,
        vk: VillageKey,
        plus: i64,
    ) -> QueryResult<Resource> {
        let target = resources::table.find((rt, vk.num()));
        diesel::update(target)
            .set(resources::amount.eq(resources::amount + plus))
            .get_result(self.dbconn())
    }
    pub fn add_karma(&self, p: PlayerKey, plus: i64) -> QueryResult<Player> {
        let target = players::table.find(p.num());
        diesel::update(target)
            .set(players::karma.eq(players::karma + plus))
            .get_result(self.dbconn())
    }
    pub fn insert_building(&self, new_building: &NewBuilding) -> Building {
        diesel::insert_into(buildings::dsl::buildings)
            .values(new_building)
            .get_result(self.dbconn())
            .expect("Inserting building")
    }
    pub fn delete_building(&self, building: &Building) {
        diesel::delete(buildings::table.filter(buildings::id.eq(building.id)))
            .execute(self.dbconn())
            .expect("Deleting building");
    }
    pub fn set_building_level(&self, building_id: BuildingKey, level: i32) {
        diesel::update(buildings::table.filter(buildings::id.eq(building_id.num())))
            .set(buildings::lv.eq(level))
            .execute(self.dbconn())
            .expect("Set building level");
    }
    pub fn insert_task(&self, task: &NewTask) -> Task {
        diesel::insert_into(tasks::dsl::tasks)
            .values(task)
            .get_result(self.dbconn())
            .expect("Inserting task")
    }

    pub fn insert_tasks(&self, tasks: &[NewTask]) -> Vec<Task> {
        diesel::insert_into(tasks::dsl::tasks)
            .values(tasks)
            .get_results(self.dbconn())
            .expect("Inserting tasks")
    }
    pub fn update_task(&self, t: &Task) {
        diesel::update(t)
            .set(t)
            .execute(self.dbconn())
            .expect("Updating task");
    }
    pub fn delete_task(&self, task: &Task) {
        diesel::delete(tasks::table.filter(tasks::id.eq(task.id)))
            .execute(self.dbconn())
            .expect("Deleting task");
    }
    pub fn flush_task_queue(&self, worker_id: WorkerKey) {
        diesel::delete(tasks::table.filter(tasks::worker_id.eq(worker_id.num())))
            .filter(tasks::start_time.gt(diesel::dsl::now.at_time_zone("UTC")))
            .execute(self.dbconn())
            .expect("Deleting task");
    }
    pub fn insert_streams(&self, streams: &[NewStream]) -> Vec<Stream> {
        diesel::insert_into(streams::dsl::streams)
            .values(streams)
            .get_results(self.dbconn())
            .expect("Inserting streams")
    }
    pub fn insert_villages(&self, villages: &[NewVillage]) -> Vec<Village> {
        diesel::insert_into(villages::dsl::villages)
            .values(villages)
            .get_results(self.dbconn())
            .expect("Inserting villages")
    }
    pub fn insert_ability(&self, a: &NewAbility) -> Ability {
        diesel::insert_into(abilities::dsl::abilities)
            .values(a)
            .get_result(self.dbconn())
            .expect("Inserting ability")
    }
    pub fn insert_effect(&self, e: &NewEffect) -> Effect {
        diesel::insert_into(effects::dsl::effects)
            .values(e)
            .get_result(self.dbconn())
            .expect("Inserting effect")
    }
    pub fn update_ability_used_timestamp(&self, worker: WorkerKey, at: AbilityType) {
        let target = abilities::table.find((at, worker.num()));
        diesel::update(target)
            .set(abilities::last_used.eq(diesel::dsl::now.at_time_zone("UTC").nullable()))
            .execute(self.dbconn())
            .expect("Updating ability timestamp");
    }
    pub fn insert_worker_flag(&self, wf: WorkerFlag) {
        diesel::insert_into(worker_flags::dsl::worker_flags)
            .values(wf)
            .execute(self.dbconn())
            .expect("Inserting flag");
    }
    pub fn update_worker_flag_timestamp_now(&self, w: WorkerKey, f: WorkerFlagType) {
        let target = worker_flags::table.find((w.num(), f));
        diesel::update(target)
            .set(worker_flags::last_update.eq(diesel::dsl::now.at_time_zone("UTC")))
            .execute(self.dbconn())
            .expect("Updating flag timestamp to now");
    }
    pub fn update_worker_flag_timestamp(
        &self,
        w: WorkerKey,
        f: WorkerFlagType,
        ts: chrono::NaiveDateTime,
    ) {
        let target = worker_flags::table.find((w.num(), f));
        diesel::update(target)
            .set(worker_flags::last_update.eq(ts))
            .execute(self.dbconn())
            .expect("Updating flag timestamp");
    }
    pub fn worker_flags(&self, worker: WorkerKey) -> Vec<WorkerFlag> {
        worker_flags::table
            .filter(worker_flags::worker_id.eq(worker.num()))
            .get_results(self.dbconn())
            .expect("Error loading data")
    }
    pub fn insert_visit_report(&self, vr: NewVisitReport) -> VisitReport {
        diesel::insert_into(visit_reports::dsl::visit_reports)
            .values(vr)
            .get_result(self.dbconn())
            .expect("Inserting visit report")
    }
    pub fn insert_visit_report_rewards(&self, rewards: Vec<NewReward>) {
        diesel::insert_into(rewards::dsl::rewards)
            .values(rewards)
            .execute(self.dbconn())
            .expect("Inserting rewards");
    }
    pub fn delete_visit_report(&self, obj: &VisitReport) {
        let result = diesel::delete(obj).execute(self.dbconn());
        if result.is_err() {
            println!("Couldn't delete {:?}", obj);
        }
    }
    pub fn set_satisfied(&self, hid: HoboKey, aid: AttackKey, satisfied: bool) {
        let target = attacks_to_hobos::table.find((aid.num(), hid.num()));
        diesel::update(target)
            .set(attacks_to_hobos::satisfied.eq(satisfied))
            .execute(self.dbconn())
            .expect("setting satisfied");
    }
    pub fn release_resting_visitor(&self, hid: HoboKey, aid: AttackKey) {
        let target = attacks_to_hobos::table.find((aid.num(), hid.num()));
        diesel::update(target)
            .set(attacks_to_hobos::released.eq(diesel::dsl::now.at_time_zone("UTC").nullable()))
            .execute(self.dbconn())
            .expect("setting released");
    }
    pub fn assign_player_quest(&self, p: PlayerKey, q: QuestKey) -> QueryResult<usize> {
        let qtp = QuestToPlayer {
            player_id: p.num(),
            quest_id: q.num(),
        };
        diesel::insert_into(quest_to_player::dsl::quest_to_player)
            .values(qtp)
            .execute(self.dbconn())
    }
    pub fn delete_player_quest(&self, p: PlayerKey, q: QuestKey) {
        diesel::delete(
            quest_to_player::table
                .filter(quest_to_player::player_id.eq(p.0))
                .filter(quest_to_player::quest_id.eq(q.0)),
        )
        .execute(self.dbconn())
        .expect("Deleting quest association");
    }
}
