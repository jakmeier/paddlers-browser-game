use diesel::prelude::*;
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::schema::*;
use paddlers_shared_lib::models::dsl;
use super::*;

impl DB {

    pub fn delete_unit(&self, unit: &Unit) {
        let result = diesel::delete(unit).execute(self.dbconn());
        if result.is_err() {
            println!("Couldn't delete unit {:?}", unit);
        }
    }

    pub fn delete_attack(&self, atk: &Attack) {
        let result = diesel::delete(atk).execute(self.dbconn());
        if result.is_err() {
            println!("Couldn't delete attack {:?}", atk);
        }
    }

    pub fn insert_unit(&self, u: &NewUnit) -> Unit {
        diesel::insert_into(units::dsl::units)
            .values(u)
            .get_result(self.dbconn())
            .expect("Inserting unit")
    }
    pub fn update_unit(&self, u: &Unit) {
        diesel::update(u)
            .set(u)
            .execute(self.dbconn())
            .expect("Updating unit");
    }
    pub fn insert_attack(&self, new_attack: &NewAttack) -> Attack {
        diesel::insert_into(attacks::dsl::attacks)
            .values(new_attack)
            .get_result(self.dbconn())
            .expect("Inserting attack")
    }
    pub fn insert_attack_to_unit(&self, atu: &AttackToUnit) {
        diesel::insert_into(attacks_to_units::dsl::attacks_to_units)
            .values(atu)
            .execute(self.dbconn())
            .expect("Inserting attack to unit");
    }
    pub fn insert_resource(&self, res: &Resource) -> QueryResult<usize> {
        diesel::insert_into(dsl::resources)
            .values(res)
            .execute(self.dbconn())
    }
    pub fn add_resource(&self, rt: ResourceType, plus: i64) -> QueryResult<Resource> {
        let target = resources::table.filter(resources::resource_type.eq(rt));
        diesel::update(target)
            .set(resources::amount.eq(resources::amount + plus))
            .get_result(self.dbconn())
    }
    pub fn insert_building(&self, new_building: &NewBuilding) -> Building {
        diesel::insert_into(buildings::dsl::buildings)
            .values(new_building)
            .get_result(self.dbconn())
            .expect("Inserting building")
    }
    pub fn delete_building(&self, building: &Building) {
        diesel::delete(buildings::table
            .filter(buildings::id.eq(building.id)))
            .execute(self.dbconn())
            .expect("Deleting building");
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
        diesel::delete(tasks::table
            .filter(tasks::id.eq(task.id)))
            .execute(self.dbconn())
            .expect("Deleting task");
    }
    pub fn flush_task_queue(&self, unit_id: i64) {
        diesel::delete(tasks::table
            .filter(tasks::unit_id.eq(unit_id)))
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
}