// components.rs

use bevy::prelude::{Bundle, Component, SpriteSheetBundle, Timer, TimerMode};
use bevy_ecs_ldtk::{GridCoords, LdtkEntity, LdtkIntCell};

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

/// Bundle for creating an animation component.
/// Groups all necessary components for an animation component, including the list of frames and the timer.
impl Default for Animation {
    fn default() -> Self {
        Animation {
            frames: Default::default(),
            timer: Timer::from_seconds(SPRITE_ANIMATION_SPEED, TimerMode::Repeating),
        }
    }
}

/// Bundle for creating a player entity.
/// Groups all necessary components for a player entity, including sprite, grid position, and animation.
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    #[sprite_sheet_bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

/// Plugin responsible for adding map-related systems to the game.
pub struct MapPlugin;

/// Component representing a wall in the game world.
#[derive(Default, Component)]
pub struct Wall;

/// Bundle for creating a wall entity.
/// Groups all necessary components for a wall entity, primarily used for collision detection.
#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}

/// Plugin responsible for adding spell_fire-related systems to the game.
pub struct SpellFirePlugin;

/// Component representing a Spell Fire entity.
/// This component is used to identify and interact with spell_fire entities in the game world.
#[derive(Default, Component, Debug)]
pub struct SpellFire;
