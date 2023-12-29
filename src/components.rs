use bevy::prelude::{Bundle, Component, SpriteSheetBundle};
use bevy_ecs_ldtk::{GridCoords, LdtkEntity, LdtkIntCell};
// use bevy_inspector_egui::InspectorOptions;

pub struct PlayerPlugin;

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    #[sprite_sheet_bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

pub struct MapPlugin;

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}
