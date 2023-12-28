use bevy::prelude::{Bundle, Component, SpriteSheetBundle, Timer};
use bevy_ecs_ldtk::{GridCoords, LdtkEntity};
use bevy_inspector_egui::InspectorOptions;

pub struct PlayerPlugin;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default, InspectorOptions)]
pub struct Player;

#[derive(Default, Bundle, Clone, LdtkEntity, InspectorOptions)]
pub struct PlayerBundle {
    pub player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);
