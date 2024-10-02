
pub const SCREEN_MAX_X: f32 = 1920.0;
pub const SCREEN_MAX_Y: f32 = 1080.0;
pub const HORIZON_ACTUAL: f32 = 420.0; // Where the sky meets land
pub const HORIZON: f32 = HORIZON_ACTUAL - 50.0; // Where the infinity point is
pub const SCREEN_MID_X: f32 = SCREEN_MAX_X / 2.0;
pub const Z_ORIGIN_Y_OFFSET: f32 = SCREEN_MAX_Y - 200.0; // - 458.0; // Where the first layer starts
pub const LAND_PROJECTION_HEIGHT: f32 = Z_ORIGIN_Y_OFFSET - HORIZON;
pub const X_UNIT: f32 =  32.0; // Width in  pixels at z0 to separate
pub const Z_UNIT: f32 = 0.05; // Separation degree for z
pub const Y_UNIT: f32 = 40.0;

pub const SCREEN_QUART_X: f32 = SCREEN_MAX_X/4.0;

// in-game x-axis bounds within which to display
pub const CULL_WORLD_X_FULLSCREEN : f32 = 76.0; //56.0;
pub const CULL_WORLD_X_HALFSCREEN : f32 = 32.0;