use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;

use crate::components::*;
use crate::constants::*;
use crate::map::LevelWalls;
use crate::util::convert_vec3_to_vec2;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_player_from_input, animate_player))
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

fn move_player_from_input(
    mut player_query: Query<
        (&mut Transform, &mut TextureAtlasSprite, &mut GridCoords),
        With<Player>,
    >,
    time: Res<Time>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), Without<Player>>,
    input_res: Res<Input<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    let speed = PLAYER_SPRITE_SPEED * time.delta_seconds();
    let mut move_vec = Vec2::ZERO;

    // Convert input to change in GridCoords
    if input_res.pressed(KeyCode::W) {
        move_vec.y += speed;
    } else if input_res.pressed(KeyCode::A) {
        move_vec.x -= speed;
    } else if input_res.pressed(KeyCode::S) {
        move_vec.y -= speed;
    } else if input_res.pressed(KeyCode::D) {
        move_vec.x += speed;
    } else {
        // If we didn't move the player, we don't need to continue
        return;
    };

    // Assign the new destination to the player
    for (mut player_transform, mut player_sprite, mut player_grid_coords) in player_query.iter_mut()
    {
        // let destination = *player_grid_coords + movement_direction;
        let player_dest_trans =
            convert_vec3_to_vec2(player_transform.translation + move_vec.extend(0.0));

        // let mut player_adjustment = destination;
        let mut player_dest_coords =
            translation_to_grid_coords(player_dest_trans, IVec2::splat(GRID_SIZE));
        player_dest_coords.y -= 1; // Measure from the lower half of the player sprite

        if !level_walls.in_wall(&player_dest_coords) {
            *player_grid_coords = player_dest_coords;
            player_transform.translation.x = player_dest_trans.x;
            player_transform.translation.y = player_dest_trans.y;
        }

        // Make the player sprite face the right direction
        match move_vec.x {
            x if x < 0.0 => player_sprite.flip_x = true,
            x if x > 0.0 => player_sprite.flip_x = false,
            _ => {} // No change on zero
        }

        // Assign x and y of player transform to the camera (not z)
        let (_orthographic_projection, mut camera_transform) = camera_query.single_mut();
        debug!(
            "camera@{:?} player@{:?} sprite.index={:?}",
            camera_transform.translation, player_transform.translation, player_sprite.index,
        );

        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y =
            player_transform.translation.y - (WINDOW_HEIGHT / CAMERA_HEIGHT_OFFSET);
    }
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite), With<Player>>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            // Cycle through the list of animation frames
            if !animation.frames.is_empty() {
                let next_frame = (animation
                    .frames
                    .iter()
                    .position(|&f| f == sprite.index)
                    .unwrap_or(0)
                    + 1)
                    % animation.frames.len();
                sprite.index = animation.frames[next_frame];
            }
        }
    }
}
