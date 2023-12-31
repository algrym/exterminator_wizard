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

/// Speed of the player sprite.
/// This value determines how fast the player moves in the game world.
pub const PLAYER_SPRITE_SPEED: f32 = 100.0;
