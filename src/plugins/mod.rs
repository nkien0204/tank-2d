pub mod asset_loader;
pub mod camera;
pub mod collision_detection;
pub mod debug;
pub mod despawn;
pub mod enemy;
pub mod game_state;
pub mod manual;
pub mod map;
pub mod movement;
pub mod tank;

const BACKGROUND_Z_INDEX: f32 = -10.0;
const TANK_OVERLAP_Z_INDEX: f32 = 1.0;

const DEFAULT_SCALE: f32 = 0.5;
const VELOCITY_SCALAR: f32 = 100.0;
const ACCELERATION_SCALAR: f32 = 1.0;

const SHELL_SPEED: f32 = 500.0;
const SHELL_FORWARD_SPAWN_SCALAR: f32 = 30.0;
const SHELL_RADIUS: f32 = 5.0;

const ALLIES_TAG_NAME: &str = "Allies";
const ENEMIES_TAG_NAME: &str = "Enemies";
