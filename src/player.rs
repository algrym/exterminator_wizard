use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::grid_coords_to_translation;

use crate::components::*;
use crate::constants::*;
use crate::map::LevelWalls;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player_from_input)
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

fn move_player_from_input(
    mut player_query: Query<
        (&mut Transform, &mut TextureAtlasSprite, &mut GridCoords),
        With<Player>,
    >,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), Without<Player>>,
    input_res: Res<Input<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    // Convert input to change in GridCoords
    let movement_direction = if input_res.just_pressed(KeyCode::W) {
        GridCoords::new(0, 1)
    } else if input_res.just_pressed(KeyCode::A) {
        GridCoords::new(-1, 0)
    } else if input_res.just_pressed(KeyCode::S) {
        GridCoords::new(0, -1)
    } else if input_res.just_pressed(KeyCode::D) {
        GridCoords::new(1, 0)
    } else {
        // If we didn't move the player, we don't need to continue
        GridCoords::new(0, 0) // TODO: don't continue the function if no movement, its wasteful
                              //return;
    };

    // Assign the new destination to the player
    for (mut player_transform, mut player_sprite, mut player_grid_coords) in player_query.iter_mut()
    {
        let destination = *player_grid_coords + movement_direction;
        let mut player_adjustment = destination;
        player_adjustment.y -= 1; // Measure from the lower half of the player sprite
        if !level_walls.in_wall(&player_adjustment) {
            *player_grid_coords = destination;
        }

        // Make the player sprite face the right direction
        match movement_direction.x {
            x if x < 0 => player_sprite.flip_x = true,
            x if x > 0 => player_sprite.flip_x = false,
            _ => {} // No change on zero
        }

        // Update the player transform
        player_transform.translation =
            grid_coords_to_translation(*player_grid_coords, IVec2::splat(GRID_SIZE))
                .extend(player_transform.translation.z);

        // Assign x and y of player transform to the camera (not z)
        let (_orthographic_projection, mut camera_transform) = camera_query.single_mut();
        info!(
            "camera@{:?} player@{:?}",
            camera_transform.translation, player_transform.translation
        );

        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y =
            player_transform.translation.y - (WINDOW_HEIGHT / CAMERA_HEIGHT_OFFSET);
    }
}
