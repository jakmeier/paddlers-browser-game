use crate::schema::*;
use crate::{generated::QuestName, prelude::*};
use diesel::prelude::*;

pub trait GameDB {
    fn dbconn(&self) -> &PgConnection;

    fn player(&self, player_id: PlayerKey) -> Option<Player> {
        players::table
            .find(player_id.num())
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn player_by_uuid(&self, uuid: uuid::Uuid) -> Option<Player> {
        players::table
            .filter(players::uuid.eq(uuid))
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn player_by_village(&self, vid: VillageKey) -> Option<Player> {
        villages::table
            .filter(villages::id.eq(vid.num()))
            .inner_join(players::table)
            .select(players::all_columns)
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn hobo(&self, hobo_id: HoboKey) -> Option<Hobo> {
        let results = hobos::table
            .filter(hobos::id.eq(hobo_id.num()))
            .get_result::<Hobo>(self.dbconn())
            .optional()
            .expect("Error loading data");
        results
    }
    fn hobos(&self, village: VillageKey) -> Vec<Hobo> {
        let results = hobos::table
            .filter(hobos::home.eq(village.num()))
            .limit(500)
            .load::<Hobo>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn settled_hobo_count(&self, village: VillageKey) -> i64 {
        hobos::table
            .filter(hobos::home.eq(village.num()))
            .filter(diesel::dsl::not(hobos::nest.is_null()))
            .count()
            .get_result(self.dbconn())
            .expect("Error loading data")
    }
    fn worker_priv(&self, worker_id: WorkerKey) -> Option<Worker> {
        let results = workers::table
            .filter(workers::id.eq(worker_id.num()))
            .get_result::<Worker>(self.dbconn())
            .optional()
            .expect("Error loading data");
        results
    }
    fn worker_auth_by_player(&self, worker_id: WorkerKey, player_id: PlayerKey) -> Option<Worker> {
        let results = villages::table
            .inner_join(workers::table)
            .inner_join(players::table)
            .filter(players::id.eq(player_id.num()))
            .filter(workers::id.eq(worker_id.num()))
            .select(workers::all_columns)
            .first::<Worker>(self.dbconn())
            .optional()
            .expect("Error loading data");
        results
    }
    fn worker_count(&self, village: VillageKey) -> i64 {
        workers::table
            .filter(workers::home.eq(village.num()))
            .count()
            .get_result(self.dbconn())
            .expect("Error loading data")
    }
    fn workers(&self, village: VillageKey) -> Vec<Worker> {
        let results = workers::table
            .filter(workers::home.eq(village.num()))
            .limit(500)
            .load::<Worker>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn workers_with_job(&self, village: VillageKey, jobs: &[TaskType]) -> Vec<Worker> {
        let results = workers::table
            .inner_join(tasks::table)
            .filter(workers::home.eq(village.num()))
            .filter(tasks::task_type.eq_any(jobs))
            .filter(tasks::start_time.lt(diesel::dsl::now.at_time_zone("UTC")))
            .select(workers::all_columns)
            .distinct()
            .load::<Worker>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn hero(&self, village: VillageKey) -> Option<Worker> {
        // Now (0.2) there is only one worker allowed in a town, which has to be the hero.
        let result = workers::table
            .filter(workers::home.eq(village.num()))
            .first::<Worker>(self.dbconn())
            .optional()
            .expect("Error loading data");
        result
    }
    fn count_workers_at_pos_doing_job(
        &self,
        village: VillageKey,
        x: i32,
        y: i32,
        job: TaskType,
    ) -> usize {
        workers::table
            .inner_join(tasks::table)
            .filter(tasks::task_type.eq(job))
            .filter(workers::home.eq(village.num()))
            .filter(tasks::x.eq(x))
            .filter(tasks::y.eq(y))
            .select(diesel::dsl::count(workers::id))
            .first::<i64>(self.dbconn())
            .expect("Error loading data") as usize
    }
    fn attacks(&self, village: VillageKey, min_id: Option<i64>) -> Vec<Attack> {
        let results = attacks::table
            .filter(attacks::destination_village_id.eq(village.num()))
            .filter(attacks::id.ge(min_id.unwrap_or(0)))
            .order_by(attacks::arrival)
            .limit(500)
            .load::<Attack>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn attacks_that_entered(&self, village: VillageKey, min_id: Option<i64>) -> Vec<Attack> {
        let results = attacks::table
            .filter(attacks::destination_village_id.eq(village.num()))
            .filter(attacks::id.ge(min_id.unwrap_or(0)))
            .filter(
                attacks::entered_destination.le(diesel::dsl::now.at_time_zone("UTC").nullable()),
            )
            .order_by(attacks::entered_destination)
            .limit(500)
            .load::<Attack>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn attacks_count(&self, village: VillageKey, min_id: Option<i64>) -> usize {
        let results = attacks::table
            .filter(attacks::destination_village_id.eq(village.num()))
            .filter(attacks::id.ge(min_id.unwrap_or(0)))
            .select(diesel::dsl::count(attacks::id))
            .first::<i64>(self.dbconn())
            .expect("Error loading data");
        results as usize
    }
    fn attacks_not_entered_count(&self, village: VillageKey) -> usize {
        let results = attacks::table
            .filter(attacks::destination_village_id.eq(village.num()))
            .filter(attacks::entered_destination.is_null())
            .select(diesel::dsl::count(attacks::id))
            .first::<i64>(self.dbconn())
            .expect("Error loading data");
        results as usize
    }
    fn attack_hobos(&self, atk: AttackKey) -> Vec<Hobo> {
        let results = attacks_to_hobos::table
            .inner_join(hobos::table)
            .filter(attacks_to_hobos::attack_id.eq(atk.num()))
            .select(hobos::all_columns)
            .limit(500)
            .load::<Hobo>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn attack_hobos_active_with_attack_info(&self, atk: &Attack) -> Vec<(Hobo, AttackToHobo)> {
        let results = attacks_to_hobos::table
            .inner_join(hobos::table)
            .filter(attacks_to_hobos::attack_id.eq(atk.id))
            .filter(attacks_to_hobos::satisfied.is_null())
            .select((hobos::all_columns, attacks_to_hobos::all_columns))
            .limit(500)
            .load::<(Hobo, AttackToHobo)>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn attack_hobos_with_attack_info(&self, atk: &Attack) -> Vec<(Hobo, AttackToHobo)> {
        let results = attacks_to_hobos::table
            .inner_join(hobos::table)
            .filter(attacks_to_hobos::attack_id.eq(atk.id))
            .select((hobos::all_columns, attacks_to_hobos::all_columns))
            .limit(500)
            .load::<(Hobo, AttackToHobo)>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn attack_hobos_satisfied(&self, atk: &Attack) -> Vec<Hobo> {
        let results = attacks_to_hobos::table
            .inner_join(hobos::table)
            .filter(attacks_to_hobos::attack_id.eq(atk.id))
            .filter(attacks_to_hobos::satisfied.eq(true))
            .select(hobos::all_columns)
            .limit(500)
            .load::<Hobo>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn attack_done(&self, atk: &Attack) -> bool {
        self.attack_hobos_active_with_attack_info(atk).len() == 0
    }
    /// Visitors resting in town right now
    fn resting_visitors(&self, village_id: VillageKey) -> Vec<(Hobo, AttackKey)> {
        attacks_to_hobos::table
            .inner_join(attacks::table)
            .inner_join(hobos::table)
            .inner_join(villages::table.on(villages::id.eq(attacks::destination_village_id)))
            .filter(villages::id.eq(village_id.num()))
            // condition for "resting"
            .filter(hobos::hurried.eq(false))
            .filter(attacks_to_hobos::satisfied.is_null())
            .filter(
                attacks::entered_destination.le(diesel::dsl::now.at_time_zone("UTC").nullable()),
            )
            //
            .order_by(attacks::entered_destination.asc())
            .select((hobos::all_columns, attacks::id))
            .limit(500)
            .load::<(Hobo, i64)>(self.dbconn())
            .expect("Error loading data")
            .into_iter()
            .map(|(hobo, key)| (hobo, AttackKey(key)))
            .collect()
    }

    fn building(&self, building: BuildingKey) -> Option<Building> {
        buildings::table
            .filter(buildings::id.eq(building.num()))
            .first::<Building>(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn buildings(&self, village: VillageKey) -> Vec<Building> {
        let results = buildings::table
            .filter(buildings::village_id.eq(village.num()))
            .limit(500)
            .load::<Building>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn find_building_by_coordinates(
        &self,
        x: i32,
        y: i32,
        village: VillageKey,
    ) -> Option<Building> {
        let result = buildings::table
            .filter(buildings::village_id.eq(village.num()))
            .filter(buildings::x.eq(x).and(buildings::y.eq(y)))
            .first::<Building>(self.dbconn())
            .optional()
            .expect("Error loading data");
        result
    }
    fn maybe_resource(&self, r: ResourceType, v: VillageKey) -> Option<i64> {
        resources::table
            .find((r, v.num()))
            .first(self.dbconn())
            .map(|res: Resource| res.amount)
            .optional()
            .expect("Error loading data")
    }
    fn resource(&self, r: ResourceType, v: VillageKey) -> i64 {
        resources::table
            .find((r, v.num()))
            .first(self.dbconn())
            .map(|res: Resource| res.amount)
            .unwrap_or(0)
    }
    fn worker_tasks(&self, worker_id: WorkerKey) -> Vec<Task> {
        let results = tasks::table
            .filter(tasks::worker_id.eq(worker_id.num()))
            .limit(500)
            .load::<Task>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn worker_abilities(&self, worker_id: WorkerKey) -> Vec<Ability> {
        let results = abilities::table
            .filter(abilities::worker_id.eq(worker_id.num()))
            .limit(10)
            .load::<Ability>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn worker_ability(&self, worker_id: WorkerKey, ability_type: AbilityType) -> Option<Ability> {
        abilities::table
            .find((ability_type, worker_id.num()))
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn past_worker_tasks(&self, worker_id: WorkerKey) -> Vec<Task> {
        let results = tasks::table
            .filter(tasks::worker_id.eq(worker_id.num()))
            .filter(tasks::start_time.lt(diesel::dsl::now.at_time_zone("UTC")))
            .order(tasks::start_time.asc())
            .limit(500)
            .load::<Task>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn earliest_future_task(&self, worker_id: WorkerKey) -> Option<Task> {
        tasks::table
            .filter(tasks::worker_id.eq(worker_id.num()))
            .filter(tasks::start_time.ge(diesel::dsl::now.at_time_zone("UTC")))
            .order(tasks::start_time.asc())
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn current_and_next_task(&self, worker_id: WorkerKey) -> (Option<Task>, Option<Task>) {
        let mut results = tasks::table
            .filter(tasks::worker_id.eq(worker_id.num()))
            .order(tasks::start_time.asc())
            .limit(2)
            .load(self.dbconn())
            .expect("Error loading data");
        if results.len() == 1 {
            (results.pop(), None)
        } else {
            let next = results.pop();
            let current = results.pop();
            (current, next)
        }
    }
    fn current_task(&self, worker_id: WorkerKey) -> Option<Task> {
        tasks::table
            .filter(tasks::worker_id.eq(worker_id.num()))
            .filter(tasks::start_time.le(diesel::dsl::now.at_time_zone("UTC")))
            .order(tasks::start_time.asc())
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn task(&self, task_id: TaskKey) -> Option<Task> {
        tasks::table
            .find(task_id.num())
            .first(self.dbconn())
            .optional()
            .expect("Error loading task")
    }
    fn stream(&self, id: StreamKey) -> Stream {
        let result = streams::table.find(id.num()).first(self.dbconn());
        match result {
            Ok(s) => s,
            Err(e) => {
                panic!("Failed loading {:?}. Error: {}", id, e);
            }
        }
    }
    fn streams(&self, low_x: f32, high_x: f32) -> Vec<Stream> {
        let results = streams::table
            .filter(streams::start_x.ge(low_x))
            .filter(streams::start_x.le(high_x))
            .load::<Stream>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn village(&self, village: VillageKey) -> Option<Village> {
        villages::table
            .find(village.num())
            .first(self.dbconn())
            .optional()
            .expect("Error loading village")
    }
    fn village_at(&self, x: f32, y: f32) -> Option<Village> {
        villages::table
            .filter(villages::x.ge(x))
            .filter(villages::x.lt(1.0 + x))
            .filter(villages::y.ge(y))
            .filter(villages::y.lt(1.0 + y))
            .first(self.dbconn())
            .optional()
            .expect("Error looking up village from position")
    }
    fn villages(&self, low_x: f32, high_x: f32) -> Vec<Village> {
        let results = villages::table
            .filter(villages::x.ge(low_x))
            .filter(villages::x.le(high_x))
            .load::<Village>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn player_villages(&self, player_id: PlayerKey) -> Vec<Village> {
        let results = villages::table
            .filter(villages::player_id.eq(player_id.num()))
            .load::<Village>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn all_villages(&self) -> Vec<Village> {
        let results = villages::table
            .load::<Village>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn all_player_villages(&self) -> Vec<Village> {
        let results = villages::table
            .filter(villages::player_id.is_not_null())
            .load::<Village>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn village_hobos(&self, v: VillageKey) -> Vec<Hobo> {
        let results = hobos::table
            .filter(hobos::home.eq(v.num()))
            .select(hobos::all_columns)
            .limit(500)
            .load::<Hobo>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn player_quests(&self, player_id: PlayerKey) -> Vec<Quest> {
        let results = quest_to_player::table
            .inner_join(quests::table)
            .filter(quest_to_player::player_id.eq(player_id.num()))
            .limit(50)
            .select(quests::all_columns)
            .load::<Quest>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn quest_by_name(&self, q: QuestName) -> QueryResult<Quest> {
        let quest_key = q.unique_string();
        quests::table
            .filter(quests::quest_key.eq(quest_key))
            .select(quests::all_columns)
            .get_result(self.dbconn())
    }
    fn effects_on_hobo(&self, hobo: HoboKey) -> Vec<Effect> {
        let results = effects::table
            .filter(effects::hobo_id.eq(hobo.num()))
            .limit(500)
            .load::<Effect>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn hobo_attack_info(&self, hid: HoboKey) -> Vec<(Attack, AttackToHobo)> {
        attacks_to_hobos::table
            .inner_join(attacks::table)
            .filter(attacks_to_hobos::hobo_id.eq(hid.num()))
            .select((attacks::all_columns, attacks_to_hobos::all_columns))
            .get_results(self.dbconn())
            .expect("Error in lookup")
    }
    fn hobo_is_attacking(&self, hid: HoboKey) -> bool {
        diesel::select(diesel::dsl::exists(
            attacks_to_hobos::table
                .inner_join(hobos::table)
                .inner_join(attacks::table)
                .filter(hobos::id.eq(hid.num()))
                .filter(attacks::id.is_not_null()),
        ))
        .get_result(self.dbconn())
        .expect("Error in lookup")
    }
    fn idle_hobos_in_nest(&self, bid: BuildingKey) -> Vec<Hobo> {
        hobos::table
            .filter(hobos::nest.eq(Some(bid.num())))
            .select(hobos::all_columns)
            .limit(500)
            .load::<Hobo>(self.dbconn())
            .expect("Error loading data")
    }
    fn village_owned_by(&self, vid: VillageKey, uuid: uuid::Uuid) -> bool {
        diesel::select(diesel::dsl::exists(
            players::table
                .inner_join(villages::table)
                .filter(players::uuid.eq(uuid))
                .filter(villages::id.eq(vid.num())),
        ))
        .get_result(self.dbconn())
        .expect("Error in look up")
    }
    fn worker_owned_by(&self, wid: WorkerKey, uuid: uuid::Uuid) -> bool {
        diesel::select(diesel::dsl::exists(
            players::table
                .inner_join(villages::table)
                .inner_join(workers::table.on(workers::home.eq(villages::id)))
                .filter(players::uuid.eq(uuid))
                .filter(workers::id.eq(wid.num())),
        ))
        .get_result(self.dbconn())
        .expect("Error in look up")
    }
    fn player_prophets_count(&self, uuid: uuid::Uuid) -> i64 {
        players::table
            .inner_join(villages::table)
            .inner_join(hobos::table.on(hobos::home.eq(villages::id)))
            .filter(players::uuid.eq(uuid))
            .filter(hobos::color.eq(UnitColor::Prophet))
            .select(diesel::dsl::count(hobos::id))
            .first(self.dbconn())
            .expect("Error in look up")
    }
    fn player_village_count(&self, p: PlayerKey) -> i64 {
        players::table
            .inner_join(villages::table)
            .filter(players::id.eq(p.num()))
            .select(diesel::dsl::count(villages::id))
            .first(self.dbconn())
            .expect("Error in look up")
    }
    fn players_sorted_by_karma(&self, start_index: i64, limit: i64) -> Vec<Player> {
        let results = players::table
            .order_by(players::karma.desc())
            .offset(start_index)
            .limit(limit)
            .load::<Player>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn players_count(&self) -> i64 {
        let results = players::table
            .select(diesel::dsl::count(players::id))
            .first(self.dbconn())
            .expect("Error counting players");
        results
    }
    fn report(&self, id: VisitReportKey) -> Option<VisitReport> {
        visit_reports::table
            .filter(visit_reports::id.eq(id.num()))
            .first(self.dbconn())
            .optional()
            .expect("Error loading visit report")
    }
    fn reports(&self, v: VillageKey, min_id: Option<i64>) -> Vec<VisitReport> {
        let results = visit_reports::table
            .filter(visit_reports::village_id.eq(v.num()))
            .filter(visit_reports::id.ge(min_id.unwrap_or(0)))
            .order_by(visit_reports::reported.desc())
            .limit(50)
            .load::<VisitReport>(self.dbconn())
            .expect("Error loading visit reports");
        results
    }
    fn rewards(&self, vr: VisitReportKey) -> Vec<(ResourceType, i64)> {
        visit_reports::table
            .inner_join(rewards::table)
            .filter(visit_reports::id.eq(vr.num()))
            .group_by(rewards::resource_type)
            .select(
                (
                    rewards::resource_type,
                    diesel::dsl::sql::<diesel::sql_types::BigInt>(
                        "COALESCE(SUM(amount),0)::bigint",
                    ),
                ), // Diesel currently has no real support for group by
                   // Hopefully to be added in 2020, see https://github.com/diesel-rs/diesel/issues/210)
                   // Then, it may look something like that instead:
                   // (rewards::resource_type, diesel::dsl::sum(rewards::amount))
            )
            .load::<(ResourceType, i64)>(self.dbconn())
            .expect("Error loading rewards")
    }
    fn quest_res_rewards(&self, q: QuestKey) -> Vec<QuestResReward> {
        quests::table
            .inner_join(quest_res_rewards::table)
            .filter(quests::id.eq(q.num()))
            .select(quest_res_rewards::all_columns)
            .load::<QuestResReward>(self.dbconn())
            .expect("Error loading quest resource rewards")
    }
    fn quest_building_conditions(&self, q: QuestKey) -> Vec<QuestBuildingCondition> {
        quests::table
            .inner_join(quest_building_conditions::table)
            .filter(quests::id.eq(q.num()))
            .select(quest_building_conditions::all_columns)
            .load::<QuestBuildingCondition>(self.dbconn())
            .expect("Error loading quest conditions")
    }
    fn quest_res_conditions(&self, q: QuestKey) -> Vec<QuestResCondition> {
        quests::table
            .inner_join(quest_res_conditions::table)
            .filter(quests::id.eq(q.num()))
            .select(quest_res_conditions::all_columns)
            .load::<QuestResCondition>(self.dbconn())
            .expect("Error loading quest conditions")
    }
    fn quest_worker_conditions(&self, q: QuestKey) -> Vec<QuestWorkerCondition> {
        quests::table
            .inner_join(quest_worker_conditions::table)
            .filter(quests::id.eq(q.num()))
            .select(quest_worker_conditions::all_columns)
            .load::<QuestWorkerCondition>(self.dbconn())
            .expect("Error loading quest conditions")
    }
}
