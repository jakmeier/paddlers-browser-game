//! All GQL objects are defined in this module.
//! The fields of the public objects are directly defined here, too.
//! Fields of private objects are in their own module since they 
//! require no further authorization checks.
//!
//! Relevant for guaranteeing data secrecy is:
//!     1) All public objects only allow authorized field access
//!     2) Private objects are only created for authorized users.
//! 
//! This module is where that authorization happens.
//!     Each field  has either of these visibilities:
//!         same as the parent GQL object
//!         more restricted => Check necessary when reading field
//! The root query is also public and each field access there must
//! also be authorized as necessary. Private constructors in this 
//! help to argue about correctness there.
//! 
//! Other modules will NOT constraint further what fields can be read. 

use super::*;
use juniper;
use juniper::FieldResult;
use paddlers_shared_lib::sql_db::keys::SqlKey;

// Complete list of fully public objects without private sub fields.
pub struct GqlMapSlice { pub low_x: i32, pub high_x: i32 }
pub struct GqlStream( pub paddlers_shared_lib::models::Stream );
pub struct GqlHobo( pub paddlers_shared_lib::models::Hobo ); // Note: Consider changing this to private

// Complete list of public objects with restricted fields access.
pub struct GqlPlayer(pub paddlers_shared_lib::models::Player);
pub struct GqlVillage(pub paddlers_shared_lib::models::Village);

// Complete list of private objects. 
// Once these are created, all sub-fields are visible!
// Note: These constructors are all private and ensure that only this module 
//       (and potentially children of it) can create these objects.
struct PrivacyGuard;
pub struct GqlAbility(pub paddlers_shared_lib::models::Ability, PrivacyGuard);
pub struct GqlAttack(pub paddlers_shared_lib::models::Attack, PrivacyGuard);
pub struct GqlBuilding(pub paddlers_shared_lib::models::Building, PrivacyGuard);
pub struct GqlEffect(pub paddlers_shared_lib::models::Effect, PrivacyGuard);
pub struct GqlTask(pub paddlers_shared_lib::models::Task, PrivacyGuard);
pub struct GqlWorker(pub paddlers_shared_lib::models::Worker, PrivacyGuard);


#[juniper::object (Context = Context)]
impl GqlPlayer {
    /// Field Visibility: public
    fn display_name(&self) -> &str {
        &self.0.display_name
    }
    /// Field Visibility: public
    fn karma(&self, ctx: &Context) -> FieldResult<i32> {
        Ok(self.0.karma as i32)
    }
    /// Field Visibility: public
    fn villages(&self, ctx: &Context) -> Vec<GqlVillage> {
        ctx.db()
            .player_villages(PlayerKey(self.0.id))
            .into_iter()
            .map(|t| GqlVillage(t))
            .collect()
    }
    /// Field Visibility: public
    fn village_count(&self, ctx: &Context) -> i32 {
        ctx.db().player_village_count(PlayerKey(self.0.id)) as i32
    }
    /// Number of (hobo) prophets that are currently owned by the player
    /// Field Visibility: user
    fn prophet_count(&self, ctx: &Context) -> FieldResult<i32> {
        ctx.check_user_key(self.0.key())?;
        Ok(ctx.db().player_prophets_count(self.0.uuid) as i32)
    }
}

#[juniper::object (Context = Context)]
impl GqlVillage {
    /// Field Visibility: public
    fn id(&self) -> i32 {
        self.0.id as i32
    }
    /// Field Visibility: public
    fn x(&self) -> f64 {
        self.0.x as f64
    }
    /// Field Visibility: public
    fn y(&self) -> f64 {
        self.0.y as f64
    }
    /// Field Visibility: public
    fn faith(&self) -> i32 {
        self.0.faith as i32
    }
    /// Field Visibility: user
    fn sticks(&self, ctx: &Context) -> FieldResult<i32> {
        ctx.check_village_key(self.0.key())?;
        Ok(
            ctx.db().resource(ResourceType::Sticks, self.0.key()) as i32
        )
    }
    /// Field Visibility: user
    fn feathers(&self, ctx: &Context) -> FieldResult<i32> {
        ctx.check_village_key(self.0.key())?;
        Ok(
            ctx.db().resource(ResourceType::Feathers, self.0.key()) as i32
        )
    }
    /// Field Visibility: user
    fn logs(&self, ctx: &Context) -> FieldResult<i32> {
        ctx.check_village_key(self.0.key())?;
        Ok(
            ctx.db().resource(ResourceType::Logs, self.0.key()) as i32
        )
    }
    /// Field Visibility: user
    fn workers(&self, ctx: &Context) -> FieldResult<Vec<GqlWorker> >{
        ctx.check_village_key(self.0.key())?;
        Ok(
            ctx.db()
                .workers(self.0.key())
                .into_iter()
                .map(GqlWorker::authorized)
                .collect()
        )
    }
    /// Field Visibility: user
    fn buildings(&self, ctx: &Context) -> FieldResult<Vec<GqlBuilding>> {
        ctx.check_village_key(self.0.key())?;
        Ok(ctx
            .db()
            .buildings(self.0.key())
            .into_iter()
            .map(GqlBuilding::authorized)
            .collect())
    }
    #[graphql(arguments(min_id(
        description = "Response only contains attacks with id >= min_id",
    )))]
    /// Field Visibility: user
    fn attacks(&self, ctx: &Context, min_id: Option<i32>) -> FieldResult<Vec<GqlAttack>> {
        ctx.check_village_key(self.0.key())?;
        Ok(ctx
            .db()
            .attacks(self.0.key(), min_id.map(i64::from))
            .into_iter()
            .map(GqlAttack::authorized)
            .collect())
    }
    /// Field Visibility: public
    fn owner(&self, ctx: &Context) -> FieldResult<Option<GqlPlayer>> {
        Ok(
            if let Some(owner) = self.0.player_id {
                let key = PlayerKey(owner as i64);
                let player = ctx.db().player(key).ok_or("Invalid owner key on village")?;
                Some(GqlPlayer(player))
            } else {
                None
            }
        )
    }
}

#[juniper::object (Context = Context)]
impl GqlHobo {
    /// Field Visibility: public
    pub fn id(&self) -> juniper::ID {
        self.0.id.to_string().into()
    }
    /// Field Visibility: public
    pub fn color(&self) -> &Option<paddlers_shared_lib::models::UnitColor> {
        &self.0.color
    }
    /// Field Visibility: public
    // TODO: Proper type handling
    pub fn hp(&self) -> i32 {
        self.0.hp as i32
    }
    /// Field Visibility: public
    // TODO: Proper type handling
    pub fn speed(&self) -> f64 {
        self.0.speed as f64
    }
    /// Field Visibility: public
    pub fn effects(&self, ctx: &Context) -> Vec<GqlEffect> {
        ctx.db()
            .effects_on_hobo(HoboKey(self.0.id))
            .into_iter()
            .map(GqlEffect::authorized )
            .collect()
    }
}

/**
 * Map data
 */

#[juniper::object (Context = Context)]
impl GqlMapSlice {
    /// Field Visibility: public
    fn streams(&self, ctx: &Context) -> Vec<GqlStream> {
        ctx.db()
            .streams(self.low_x as f32, self.high_x as f32)
            .into_iter()
            .map(|t| GqlStream(t))
            .collect()
    }
    /// Field Visibility: public
    fn villages(&self, ctx: &Context) -> Vec<GqlVillage> {
        ctx.db()
            .villages(self.low_x as f32, self.high_x as f32)
            .into_iter()
            .map(|t| GqlVillage(t))
            .collect()
    }
}

#[juniper::object (Context = Context)]
impl GqlStream {
    // TODO f32 instead of f64
    /// Field Visibility: public
    fn control_points(&self) -> Vec<f64> {
        let mut vec = vec![self.0.start_x as f64, 5.5];
        // vec.extend_from_slice(&self.0.control_points)
        vec.extend(self.0.control_points.iter().map(|f| *f as f64));
        vec
    }
}

/*
 * Constructors to use after authorization.
 * Secrecy model only works if these are only called properly!
 */
impl GqlAbility {
    pub (super) 
    fn authorized(inner: paddlers_shared_lib::models::Ability) -> Self {
        GqlAbility(inner, PrivacyGuard)
    }
}
impl GqlAttack {
    pub (super) 
    fn authorized(inner: paddlers_shared_lib::models::Attack) -> Self {
        GqlAttack(inner, PrivacyGuard)
    }
}
impl GqlBuilding {
    pub (super) 
    fn authorized(inner: paddlers_shared_lib::models::Building) -> Self {
        GqlBuilding(inner, PrivacyGuard)
    }
}
impl GqlEffect {
    pub (super) 
    fn authorized(inner: paddlers_shared_lib::models::Effect) -> Self {
        GqlEffect(inner, PrivacyGuard)
    }
}
impl GqlTask {
    pub (super) 
    fn authorized(inner: paddlers_shared_lib::models::Task) -> Self {
        GqlTask(inner, PrivacyGuard)
    }
}
impl GqlWorker {
    pub (in crate::graphql) 
    fn authorized(inner: paddlers_shared_lib::models::Worker) -> Self {
        GqlWorker(inner, PrivacyGuard)
    }
}