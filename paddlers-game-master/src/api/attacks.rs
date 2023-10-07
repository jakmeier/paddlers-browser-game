use super::check_owns_village0;
use crate::game_master::attack_funnel::PlannedAttack;
use crate::game_master::event::Event;
use crate::game_master::town_worker::TownWorkerEventMsg;
use crate::StringError;
use crate::{authentication::Authentication, game_master::story_worker::StoryWorkerMessage};
use actix_web::{web, HttpResponse};
use paddlers_shared_lib::{
    api::attacks::{AttackDescriptor, InvitationDescriptor, StartFightRequest},
    civilization::CivilizationPerk,
};
use paddlers_shared_lib::{prelude::*, story::story_trigger::StoryTrigger};

pub(crate) async fn create_attack(
    pool: web::Data<crate::db::Pool>,
    actors: web::Data<crate::ActorAddresses>,
    body: web::Json<AttackDescriptor>,
    auth: Authentication,
) -> Result<HttpResponse, StringError> {
    let pool0 = pool.clone();
    let pool1 = pool.clone();
    let attack = body.0;
    let (x, y) = attack.to;
    let from_key = attack.from;
    let home_id = from_key.num();

    let hobos = attack
        .units
        .into_iter()
        .map(move |hobo_key| {
            let db: crate::db::DB = pool.clone().get_ref().into();
            match db.hobo(hobo_key) {
                Some(hobo) => {
                    if hobo.home != home_id {
                        Err("Hobo not from this village")
                    } else {
                        Ok(hobo)
                    }
                }
                None => Err("Invalid hobo"),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    let db: crate::db::DB = pool0.get_ref().into();
    check_owns_village0(&db, &auth, from_key)?;
    let destination_village = db
        .village_at(x as f32, y as f32)
        .ok_or("Invalid target village")?;

    let db: crate::db::DB = pool1.get_ref().into();
    let origin_village = db.village(from_key).ok_or("Owned village doesn't exist")?;

    let pa = PlannedAttack {
        origin_village: Some(origin_village),
        destination_village,
        hobos,
        fixed_travel_time_s: None,
        subject_to_visitor_queue_limit: false,
    };

    actors
        .attack_funnel
        .try_send(pa)
        .or(Err("failed to send to attack funnel"))?;
    Ok(HttpResponse::Ok().into())
}

pub(crate) async fn welcome_visitor(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<StartFightRequest>,
    mut auth: Authentication,
    addr: web::Data<crate::ActorAddresses>,
) -> HttpResponse {
    let db: crate::db::DB = pool.get_ref().into();
    let destination_village = body.destination;
    let attack = body.attack;
    if !db.village_owned_by(destination_village, auth.user.uuid) {
        return HttpResponse::Forbidden().body("Village not owned by player");
    }
    if let Some(player) = auth.player_object(&db) {
        addr.story_worker.do_send(StoryWorkerMessage::new_verified(
            player.key(),
            player.story_state,
            StoryTrigger::LetVisitorIn,
        ));
    }
    db.start_fight(attack, Some(destination_village));
    HttpResponse::Ok().into()
}
pub(crate) async fn visitor_satisfied_notification(
    body: web::Json<HoboKey>,
    addr: web::Data<crate::ActorAddresses>,
) -> Result<&'static str, StringError> {
    let event = Event::CheckVisitorHp { hobo_id: body.0 };
    addr.town_worker
        .try_send(TownWorkerEventMsg(event, chrono::Utc::now()))
        .map_err(|e| format!("Send failed: {:?}", e))?;
    Ok("")
}

pub(crate) async fn new_invitation(
    pool: web::Data<crate::db::Pool>,
    body: web::Json<InvitationDescriptor>,
    mut auth: Authentication,
    addr: web::Data<crate::ActorAddresses>,
) -> Result<HttpResponse, StringError> {
    // Check that request is valid and forward request to actor
    let db: crate::db::DB = pool.get_ref().into();
    let origin_vid = db.building(body.nest).ok_or("Nest not found")?.village();
    let origin_village = db.village(origin_vid);
    let destination_village = db.village(body.to).ok_or("Village not found")?;
    let hobos = db.idle_hobos_in_nest(body.nest);
    if !auth
        .player_object(&db)
        .ok_or_else(|| "No such player".to_owned())?
        .civilization_perks()
        .has(CivilizationPerk::Invitation)
    {
        return Err("Invitations not unlocked".into());
    }
    if !db.village_owned_by(destination_village.key(), auth.user.uuid) {
        return Err("Village not owned by player".into());
    }
    let atk = PlannedAttack {
        origin_village,
        destination_village,
        hobos,
        fixed_travel_time_s: None,
        subject_to_visitor_queue_limit: true,
    };
    addr.attack_funnel
        .try_send(atk)
        .map_err(|e| format!("Spawning attack failed: {:?}", e))?;

    Ok(HttpResponse::Ok().into())
}
