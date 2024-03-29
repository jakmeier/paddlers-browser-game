use crate::db::DB;
use diesel::result::DatabaseErrorKind;
use paddlers_shared_lib::prelude::*;
use std::fmt;

impl DB {
    pub(super) fn new_player(
        &self,
        display_name: String,
        uuid: uuid::Uuid,
    ) -> Result<Player, PlayerCreationError> {
        let player = NewPlayer {
            display_name: display_name,
            karma: 0,
            uuid,
        };
        let player = self.insert_player(&player).map_err(|err| {
            if let diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _info) =
                err
            {
                PlayerCreationError::AlreadyExists
            } else {
                PlayerCreationError::DieselError(err)
            }
        })?;
        let village = self
            .new_village(player.key())
            .map_err(PlayerCreationError::Other)?;

        self.insert_hero(village.key());
        Ok(player)
    }

    fn insert_hero(&self, vid: VillageKey) -> Worker {
        let (x, y) = (5, 2);
        let worker = NewWorker {
            unit_type: UnitType::Hero,
            x: x,
            y: y,
            color: None,
            speed: 0.5,
            home: vid.num(),
            mana: Some(0),
            level: 1,
            exp: 0,
        };
        let worker = self.insert_worker(&worker);
        let task = NewTask {
            worker_id: worker.id,
            task_type: TaskType::Idle,
            x: x,
            y: y,
            start_time: None,
            target_hobo_id: None,
        };
        self.insert_task(&task);
        let work_ability = NewAbility {
            worker_id: worker.id,
            ability_type: AbilityType::Work,
        };
        self.insert_ability(&work_ability);
        let welcome_ability = NewAbility {
            worker_id: worker.id,
            ability_type: AbilityType::Welcome,
        };
        self.insert_ability(&welcome_ability);
        self.insert_worker_flag(WorkerFlag {
            worker_id: worker.id,
            flag_type: WorkerFlagType::ManaRegeneration,
            last_update: chrono::Utc::now().naive_utc(),
        });
        self.insert_worker_flag(WorkerFlag {
            worker_id: worker.id,
            flag_type: WorkerFlagType::Work,
            last_update: chrono::Utc::now().naive_utc(),
        });
        worker
    }

    fn new_village(&self, pid: PlayerKey) -> Result<Village, &'static str> {
        let village = self.add_village(pid)?;
        self.insert_initial_resources(village.key());
        Ok(village)
    }

    fn insert_initial_resources(&self, vid: VillageKey) {
        self.init_resources(vid);

        self.add_resource(ResourceType::Feathers, vid, 50)
            .expect("Adding initial resources");
        self.add_resource(ResourceType::Sticks, vid, 50)
            .expect("Adding initial resources");
        #[cfg(debug_assertions)]
        self.insert_dev_resources(vid);
    }
    #[cfg(debug_assertions)]
    fn insert_dev_resources(&self, vid: VillageKey) {
        self.add_resource(ResourceType::Feathers, vid, 1000)
            .expect("Adding dev resources");
        self.add_resource(ResourceType::Sticks, vid, 1000)
            .expect("Adding dev resources");
        self.add_resource(ResourceType::Logs, vid, 1000)
            .expect("Adding dev resources");
    }
}

#[derive(Debug)]
pub(super) enum PlayerCreationError {
    AlreadyExists,
    DieselError(diesel::result::Error),
    Other(&'static str),
}

impl fmt::Display for PlayerCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerCreationError::AlreadyExists => {
                write!(f, "Player already exists in the database.")
            }
            PlayerCreationError::DieselError(e) => write!(f, "{e}"),
            PlayerCreationError::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for PlayerCreationError {}
