use super::animation::{AnimatedObjectDef, AnimationVariantDef};
use paddlers_shared_lib::specification_types::SingleSprite;

pub const ANIMATION_NUM: usize = 1;
pub const ANIMATION_DEFS: [AnimatedObjectDef; ANIMATION_NUM] = [AnimatedObjectDef {
    up: AnimationVariantDef::Animated("ducks/animations/roger_back.png"),
    left: AnimationVariantDef::Animated("ducks/animations/roger_left.png"),
    down: AnimationVariantDef::Animated("ducks/animations/roger_front.png"),
    standing: AnimationVariantDef::Static("ducks/roger.png"),
    cols: 20,
    rows: 5,
    alternative: SingleSprite::Roger,
}];
