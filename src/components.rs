// components.rs

use std::time::Duration;

use bevy::prelude::{Bundle, Component, SpriteSheetBundle, Timer};
use bevy::time::TimerMode;
use bevy_ecs_ldtk::{GridCoords, LdtkEntity, LdtkIntCell};
use bevy_rapier2d::prelude::*;

use crate::constants::*;

/// Plugin responsible for adding player-related systems to the game.
pub struct PlayerPlugin;

/// Component representing the player entity.
/// This component is used to identify and interact with the player in the game world.
#[derive(Default, Component, Debug)]
pub struct Player;

/// Component for handling sprite animation.
///
/// Contains a list of frame indices for the animation and a timer to control the
/// frame rate of the animation.
#[derive(Component)]
pub struct Animation {
    /// Indices of the frames in the sprite sheet used for animation.
    pub frames: Vec<usize>,
    /// Timer to control when the frame should be updated.
    pub timer: Timer,
}

impl Default for Animation {
    /// Provides a default set of frames and a timer for the animation.
    fn default() -> Self {
        Animation {
            frames: PLAYER_SPRITE_FRAMES.to_vec(),
            timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
        }
    }
}

/// Bundle for creating a player entity.
/// Groups all necessary components for a player entity, including sprite, grid position, and animation.
#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    #[sprite_sheet_bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
    pub animation: Animation,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Default::default(),
            sprite_bundle: Default::default(),
            grid_coords: Default::default(),
            animation: Default::default(),
            rigid_body: RigidBody::Dynamic, // Makes the player affected by physics
            collider: Collider::cuboid(PLAYER_SPRITE_WIDTH / 2.0, PLAYER_SPRITE_HEIGHT / 2.0),
        }
    }
}

/// Plugin responsible for adding map-related systems to the game.
pub struct MapPlugin;

/// Component representing a wall in the game world.
#[derive(Default, Component)]
pub struct Wall;

/// Bundle for creating a wall entity.
/// Groups all necessary components for a wall entity, primarily used for collision detection.
#[derive(Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

impl Default for WallBundle {
    fn default() -> Self {
        WallBundle {
            wall: Default::default(),
            rigid_body: RigidBody::Fixed, // Walls are static
            collider: Collider::cuboid(WALL_SPRITE_WIDTH / 2.0, WALL_SPRITE_HEIGHT / 2.0),
        }
    }
}
