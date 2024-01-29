// constants.rs

/// Filename of the LDtk map used in the game.
pub const MAP_FILENAME: &str = "map.ldtk";

/// Size of each grid cell in the map, in pixels.
pub const GRID_SIZE: i32 = 16;

/// Width of the game window, in pixels.
pub const WINDOW_WIDTH: f32 = 1280.0;

/// Height of the game window, in pixels.
pub const WINDOW_HEIGHT: f32 = 720.0;

/// Scale factor for the camera.
/// This value affects how much of the game world is visible on the screen.
pub const CAMERA_SCALE: f32 = 0.5;

/// Vertical offset for the camera relative to the player.
/// Adjusts the camera's height position when following the player.
pub const CAMERA_HEIGHT_OFFSET: f32 = 1.5; // TODO: This is bogus. How does camera x,y work?

/// Dimensions for the wall sprites (16, 16)
pub const WALL_SPRITE_WIDTH: f32 = GRID_SIZE as f32;
pub const WALL_SPRITE_HEIGHT: f32 = GRID_SIZE as f32;

/// Dimensions for the player sprite (16, 32)
pub const PLAYER_SPRITE_WIDTH: f32 = GRID_SIZE as f32;
pub const PLAYER_SPRITE_HEIGHT: f32 = 2.0 * GRID_SIZE as f32;

/// Speed of the player sprite.
/// This value determines how fast the player moves in the game world.
pub const PLAYER_SPRITE_SPEED: f32 = 100.0;

/// List of player animation frame indexes
/// This is the list of frame indexes that will be iterated through to show animation.
/// TODO: PLAYER_SPRITE_FRAMES needs to be loaded from the LDTK player entity metadata.
pub const PLAYER_SPRITE_FRAMES: [usize; 9] = [136, 137, 138, 139, 140, 141, 142, 143, 144];

pub const _SPELL_FIRE_SPRITE_WIDTH: f32 = GRID_SIZE as f32;
pub const _SPELL_FIRE_SPRITE_HEIGHT: f32 = GRID_SIZE as f32;

/// Speed of the player sprite animation.
/// This value determines the delay between player sprite animation frames.
pub const SPRITE_ANIMATION_SPEED: f32 = 0.1;

/// Speed of the spell_fire sprite.
pub const SPELL_FIRE_SPEED: f32 = 2.0;
