use bevy::prelude::{Bundle, Component, SpriteSheetBundle, Timer};
use bevy_ecs_ldtk::{GridCoords, LdtkEntity};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

