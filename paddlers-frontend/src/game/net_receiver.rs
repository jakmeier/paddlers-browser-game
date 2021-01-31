use super::*;
use crate::game::{
    components::*, toplevel::Signal, town::TownContext, town_resources::TownResources,
    units::hobos::insert_hobos, units::worker_factory::create_worker_entities,
    units::workers::Worker, visits::attacks::Attack,
};
use crate::net::game_master_api::{HttpCreatePlayer, RestApiState};
use crate::net::graphql::query_types::{
    AttacksResponse, BuildingsResponse, HobosQueryResponse, VolatileVillageInfoResponse,
    WorkerResponse,
};
use crate::net::NetMsg;
use crate::prelude::*;
use paddle::LoadScheduler;
use paddlers_shared_lib::prelude::*;
use specs::prelude::*;
use std::convert::TryInto;
use std::sync::mpsc::TryRecvError;

pub fn loading_update_net(
    net_chan: &mut Receiver<NetMsg>,
    progress: &mut LoadScheduler,
) -> PadlResult<()> {
    match net_chan.try_recv() {
        Ok(msg) => match msg {
            NetMsg::Error(e) => match e.err {
                PadlErrorCode::GraphQlResponseError(PadlApiError::PlayerNotCreated) => {
                    nuts::send_to::<RestApiState, _>(HttpCreatePlayer);
                }
                _ => {
                    println!("Network Error: {}", e);
                }
            },
            NetMsg::Workers(response, _vid) => {
                progress.add_progress(response);
            }
            NetMsg::Player(player_info) => {
                progress.add_progress(player_info);
            }
            NetMsg::Buildings(response) => {
                progress.add_progress(response);
            }
            NetMsg::Hobos(hobos, _vid) => {
                progress.add_progress(hobos);
            }
            NetMsg::Attacks(response) => {
                progress.add_progress(response);
            }
            NetMsg::VillageInfo(response) => {
                progress.add_progress(response);
            }
            NetMsg::Leaderboard(offset, list) => {
                progress.add_progress::<NetMsg>(NetMsg::Leaderboard(offset, list));
            }
            NetMsg::Reports(data) => {
                progress.add_progress(data);
            }
            NetMsg::Quests(data) => {
                progress.add_progress(data);
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

impl Game {
    pub fn update_net(&mut self) -> PadlResult<()> {
        match self.net.try_recv() {
            Ok(msg) => {
                // println!("Received Network data!");
                match msg {
                    NetMsg::Error(e) => {
                        println!("Network Error: {}", e);
                    }
                    NetMsg::Attacks(response) => {
                        self.load_attacking_hobos(response)?;
                        self.check_resting_queue()?;
                        self.update_inflight_attack_count()?;
                    }
                    NetMsg::Buildings(response) => {
                        self.load_buildings_from_net_response(response)?;
                    }
                    NetMsg::Hobos(hobos, vid) => {
                        let ctx = self.maybe_town_context_mut(vid, "villages")?;
                        load_hobos_from_net_response(ctx, hobos)?;
                    }
                    NetMsg::Leaderboard(offset, list) => {
                        paddle::share(NetMsg::Leaderboard(offset, list));
                    }
                    NetMsg::Map(data, min, max) => {
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
                        let (map, world) = (self.map.as_mut(), &mut self.world);
                        map.map(|map| map.add_segment(world, streams, villages, min, max));
                    }
                    NetMsg::Player(player_info) => {
                        self.load_player_info(player_info)?;
                    }
                    NetMsg::VillageInfo(response) => {
                        self.load_village_info(response)?;
                        paddle::share(Signal::ResourcesUpdated);
                    }
                    NetMsg::Workers(response, vid) => {
                        let ctx = self.maybe_town_context_mut(vid, "workers")?;
                        flush_workers(ctx.world())?;
                        ctx.world_mut().maintain();
                        load_workers_from_net_response(ctx, response);
                    }
                    NetMsg::UpdateWorkerTasks(unit) => {
                        let entity = self.worker_entity_by_net_id(unit.id.parse().unwrap())?;
                        let world = self.town_world_mut();
                        let workers = &mut world.write_storage::<Worker>();
                        let worker = workers.get_mut(entity).unwrap();
                        worker.tasks.clear();

                        let net = world.read_storage::<NetObj>();
                        let ent = world.entities();
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
                    NetMsg::Reports(data) => {
                        paddle::share(NetMsg::Reports(data));
                    }
                    NetMsg::Quests(data) => {
                        paddle::share(NetMsg::Quests(data));
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
impl Game {
    pub fn load_buildings_from_net_response(&mut self, data: BuildingsResponse) -> PadlResult<()> {
        if let Some(ctx) = self.town_context.context_by_key_mut(data.village_id()) {
            let world = ctx.world_mut();
            flush_buildings(world)?;
            world.maintain();
            data.create_entities(self.town_context.active_context_mut());
        } else {
            return PadlErrorCode::DataForInactiveTownReceived("buildings").dev();
        }
        Ok(())
    }
    pub fn load_attacking_hobos(&mut self, data: AttacksResponse) -> PadlResult<()> {
        for atk in data.village.attacks {
            atk.create_entities(self)?;
        }
        Ok(())
    }
    pub fn update_inflight_attack_count(&mut self) -> PadlResult<()> {
        let attacks = self.world.read_component::<Attack>();
        let mut n = 0;
        for (a,) in (&attacks,).join() {
            if !a.has_arrived() {
                n += 1;
            }
        }
        self.inflight_visitor_groups = n;
        println!("inflight_visitor_groups = {}", n);
        Ok(())
    }
    pub fn load_village_info(&mut self, data: VolatileVillageInfoResponse) -> PadlResult<()> {
        self.town_mut().faith = data.village.faith.try_into().map_err(|_| {
            PadlError::dev_err(PadlErrorCode::InvalidGraphQLData("Faith does not fit u8"))
        })?;
        self.town_world().fetch_mut::<TownResources>().update(data);
        Ok(())
    }
    pub fn load_player_info(&mut self, player_info: PlayerInfo) -> PadlResult<()> {
        self.world.insert(player_info.clone());
        self.town_world_mut().insert(DefaultShop::new(&player_info));
        self.town_world_mut().insert(player_info);
        paddle::share(Signal::PlayerInfoUpdated);
        Ok(())
    }
    fn maybe_town_context_mut(
        &mut self,
        vid: VillageKey,
        what: &'static str,
    ) -> PadlResult<&mut TownContext> {
        self.town_context
            .context_by_key_mut(vid)
            .ok_or(PadlError::dev_err(
                PadlErrorCode::DataForInactiveTownReceived(what),
            ))
    }
}
pub fn load_workers_from_net_response(ctx: &mut TownContext, response: WorkerResponse) {
    let world = ctx.world_mut();
    let now = world.read_resource::<Now>().0;
    let results = create_worker_entities(&response, world, now);
    for res in results.into_iter() {
        if let Err(e) = res {
            nuts::publish(e);
        }
    }
}
/// Home hobos (not attackers) are loaded
pub fn load_hobos_from_net_response(
    ctx: &mut TownContext,
    hobos: HobosQueryResponse,
) -> PadlResult<()> {
    let world = ctx.world_mut();
    flush_home_hobos(world)?;
    insert_hobos(ctx, hobos)?;
    Ok(())
}
