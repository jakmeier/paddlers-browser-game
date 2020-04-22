use crate::game::{
    components::*, town::new_temple_menu, units::worker_factory::create_worker_entities,
    units::workers::Worker,
};
use crate::init::loading::LoadingState;
use crate::init::quicksilver_integration::{GameState, Signal};
use crate::net::graphql::query_types::WorkerResponse;
use crate::net::graphql::query_types::{BuildingsResponse, HobosQueryResponse};
use crate::net::NetMsg;
use crate::prelude::*;
use std::convert::TryInto;
use std::sync::mpsc::TryRecvError;

use super::*;
use specs::prelude::*;

impl LoadingState {
    pub fn update_net(&mut self) -> PadlResult<()> {
        match self.base.net_chan.try_recv() {
            Ok(msg) => match msg {
                NetMsg::Error(e) => match e.err {
                    PadlErrorCode::UserNotInDB => {
                        self.base.rest.http_create_player()?;
                    }
                    _ => {
                        println!("Network Error: {}", e);
                    }
                },
                NetMsg::Workers(response) => {
                    self.progress.report_progress_for(&response, 1);
                    self.game_data.worker_response = Some(response);
                }
                NetMsg::Player(player_info) => {
                    self.progress.report_progress_for(&player_info, 1);
                    self.game_data.player_info = Some(player_info);
                }
                NetMsg::Buildings(response) => {
                    self.progress.report_progress_for(&response, 1);
                    self.game_data.buildings_response = Some(response);
                }
                NetMsg::Hobos(hobos) => {
                    self.progress.report_progress_for(&hobos, 1);
                    self.game_data.hobos_response = Some(hobos);
                }
                NetMsg::Leaderboard(offset, list) => {
                    self.viewer_data
                        .push(PadlEvent::Network(NetMsg::Leaderboard(offset, list)));
                    self.progress
                        .report_progress::<PadlEvent>(self.viewer_data.len());
                }
                other => {
                    println!(
                        "Unexpected network message before complete initialization {:?}",
                        other,
                    );
                }
            },
            Err(TryRecvError::Disconnected) => {
                return PadlErrorCode::NoNetwork.usr();
            }
            Err(TryRecvError::Empty) => {}
        }
        Ok(())
    }
}

impl GameState {
    pub fn update_net(&mut self) -> PadlResult<()> {
        match self.game.net.try_recv() {
            Ok(msg) => {
                // println!("Received Network data!");
                match msg {
                    NetMsg::Error(e) => {
                        println!("Network Error: {}", e);
                    }
                    NetMsg::Attacks(response) => {
                        if let Some(data) = response.data {
                            for atk in data.village.attacks {
                                atk.create_entities(&mut self.game.world)?;
                            }
                        } else {
                            println!("No data returned");
                        }
                    }
                    NetMsg::Buildings(response) => {
                        self.game.load_buildings_from_net_response(response)?;
                    }
                    NetMsg::Hobos(hobos) => {
                        self.game.load_hobos_from_net_response(hobos)?;
                    }
                    NetMsg::Leaderboard(offset, list) => {
                        self.viewer.global_event(
                            &mut self.game,
                            &PadlEvent::Network(NetMsg::Leaderboard(offset, list)),
                        )?;
                    }
                    NetMsg::Map(response, min, max) => {
                        if let Some(data) = response.data {
                            let streams = data
                                .map
                                .streams
                                .iter()
                                .map(|s| {
                                    s.control_points
                                        .chunks(2)
                                        .map(|slice| (slice[0] as f32, slice[1] as f32))
                                        .collect()
                                })
                                .collect();
                            let villages = data
                                .map
                                .villages
                                .into_iter()
                                .map(VillageMetaInfo::from)
                                .collect();
                            let (map, world) = (self.game.map.as_mut(), &mut self.game.world);
                            map.map(|map| map.add_segment(world, streams, villages, min, max));
                        } else {
                            println!("No map data available");
                        }
                    }
                    NetMsg::Player(player_info) => {
                        if let Some(temple) = self.game.town().temple {
                            let mut menus = self.game.world.write_storage::<UiMenu>();
                            // This insert overwrites existing entries
                            menus
                                .insert(temple, new_temple_menu(&player_info))
                                .map_err(|_| {
                                    PadlError::dev_err(PadlErrorCode::EcsError(
                                        "Temple menu insertion failed",
                                    ))
                                })?;
                        }
                        *self.game.world.write_resource() = DefaultShop::new(player_info.karma());
                        *self.game.world.write_resource() = player_info;
                    }
                    NetMsg::VillageInfo(response) => {
                        if let Some(data) = response.data {
                            self.game.town_mut().faith =
                                data.village.faith.try_into().map_err(|_| {
                                    PadlError::dev_err(PadlErrorCode::InvalidGraphQLData(
                                        "Faith does not fit u8",
                                    ))
                                })?;
                            self.game.resources.update(data);
                            self.viewer.event(
                                &mut self.game,
                                &PadlEvent::Signal(Signal::ResourcesUpdated),
                            )?;
                        } else {
                            println!("No resources available");
                        }
                    }
                    NetMsg::Workers(response) => {
                        self.game.flush_workers()?;
                        self.game.world.maintain();
                        self.game.load_workers_from_net_response(response);
                    }
                    NetMsg::UpdateWorkerTasks(unit) => {
                        let entity = self
                            .game
                            .worker_entity_by_net_id(unit.id.parse().unwrap())?;
                        let workers = &mut self.game.world.write_storage::<Worker>();
                        let worker = workers.get_mut(entity).unwrap();
                        worker.tasks.clear();

                        let net = self.game.world.read_storage::<NetObj>();
                        let ent = self.game.world.entities();
                        for task in unit.tasks {
                            match task.create(&net, &ent) {
                                Ok(task) => worker.tasks.push_back(task),
                                Err(e) => {
                                    match e.err {
                                        PadlErrorCode::UnknownNetObj(_) => {
                                            // NOP: Best to ignore and move on with other tasks
                                        }
                                        _ => {
                                            return Err(e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(TryRecvError::Disconnected) => {
                return PadlErrorCode::NoNetwork.usr();
            }
            Err(TryRecvError::Empty) => {}
        }
        Ok(())
    }
}
impl<'a, 'b> Game<'a, 'b> {
    pub fn load_workers_from_net_response(&mut self, response: WorkerResponse) {
        let now = self.world.read_resource::<Now>().0;
        let results = create_worker_entities(&response, &mut self.world, now);
        let mut q = self.world.write_resource::<ErrorQueue>();
        for res in results.into_iter() {
            if let Err(e) = res {
                q.push(e);
            }
        }
    }
    pub fn load_buildings_from_net_response(
        &mut self,
        response: BuildingsResponse,
    ) -> PadlResult<()> {
        if let Some(data) = response.data {
            self.flush_buildings()?;
            self.world.maintain();
            data.create_entities(self);
        } else {
            println!("No buildings available");
        }
        Ok(())
    }
    pub fn load_hobos_from_net_response(&mut self, hobos: HobosQueryResponse) -> PadlResult<()> {
        self.flush_home_hobos()?;
        self.insert_hobos(hobos)?;
        Ok(())
    }
}
