use std::time::Duration;

use bevy::prelude::{Bundle, Component, SpriteSheetBundle, Timer};
use bevy::time::TimerMode;
use bevy_ecs_ldtk::{GridCoords, LdtkEntity, LdtkIntCell};

// use bevy_inspector_egui::InspectorOptions;

pub struct PlayerPlugin;

#[derive(Default, Component)]
pub struct Player;

#[derive(Component)]
pub struct Animation {
    pub frames: Vec<usize>,
    pub timer: Timer,
}

impl Default for Animation {
    fn default() -> Self {
        Animation {
            frames: vec![136, 137, 138, 139, 140, 141, 142, 143, 144], // TODO: load this elsewhere
            timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    #[sprite_sheet_bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
    pub animation: Animation,
}

pub struct MapPlugin;

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}
