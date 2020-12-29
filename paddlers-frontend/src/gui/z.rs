
/*
 * Browser z-values (i32)
 */
 // div-rs class sets z-index to 1
pub const MENU_BG_Z_LAYER: i32 = 2;
pub const MENU_Z_LAYER: i32 = 3;

/*
 * WebGL z-values (i16)
 */
// Background [0,99]
pub const Z_TEXTURE: i16 = 0;
pub const Z_GRID: i16 = 10;
pub const Z_RIVER: i16 = 15;
pub const Z_TILE_SHADOW: i16 = 50;

// Units [100,199]
pub const Z_BUILDINGS: i16 = 100;
pub const Z_VISITOR: i16 = 110;
pub const Z_UNITS: i16 = 120;
pub const Z_UNIT_UI_HINT: i16 = 150;

// UI [200,400]
pub const Z_MENU_BOX: i16 = 220;
pub const Z_UI_BORDERS: i16 = 230;
pub const Z_HP_BAR: i16 = 250;
pub const Z_UI_MENU: i16 = 280;
pub const Z_GRABBED_ITEM: i16 = 350;
