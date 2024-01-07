// player.rs

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;

use crate::components::*;
use crate::constants::*;
use crate::map::LevelWalls;
use crate::util::convert_vec3_to_vec2;

/// PlayerPlugin is responsible for handling player-related functionalities
/// in the game. This includes processing player input for movement
/// and animating the player sprite.
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_player_from_input, animate_player, dbg_player))
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

/// Processes player input for movement.
///
/// This function updates the player's position and orientation based on keyboard inputs.
/// It ensures that the player does not move into walls and updates the camera position
/// to follow the player.
///
/// # Arguments
/// * `player_query` - Query to access player entities' transforms, sprites, and grid coordinates.
/// * `time` - Resource to get time information for frame delta calculation.
/// * `camera_query` - Query to access and update the camera's transform.
/// * `input_res` - Resource to get the current input state.
/// * `level_walls` - Resource containing information about wall locations in the level.
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
    }
    if input_res.pressed(KeyCode::A) {
        move_vec.x -= speed;
    }
    if input_res.pressed(KeyCode::S) {
        move_vec.y -= speed;
    }
    if input_res.pressed(KeyCode::D) {
        move_vec.x += speed;
    }
    // If we didn't move the player, we don't need to continue.
    // We need to run the rest of this ONE TIME to fix the camera.

    // Assign the new destination to the player
    for (mut player_transform, mut player_sprite, mut player_grid_coords) in player_query.iter_mut()
    {
        // Where is the player's planned destination, in transform domain?
        let player_dest_trans =
            convert_vec3_to_vec2(player_transform.translation + move_vec.extend(0.0));

        // Where is the player's planned destination, in coordinate domain?
        let mut player_dest_coords =
            translation_to_grid_coords(player_dest_trans, IVec2::splat(GRID_SIZE));
        player_dest_coords.y -= 1; // Measure from the lower half of the player sprite

        // If there's no collision, then copy the plans into the actual
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
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y =
            player_transform.translation.y - (WINDOW_HEIGHT / CAMERA_HEIGHT_OFFSET);
    }
}

/// Animates the player sprite based on the defined animation frames.
///
/// This function cycles through a series of sprite indices to animate the player sprite.
/// It uses a timer to control the animation speed.
///
/// # Arguments
/// * `time` - Resource to get time information for the animation timer.
/// * `query` - Query to access player entities' animations and texture atlas sprites.
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

pub fn dbg_player(
    input_res: Res<Input<KeyCode>>,
    mut query: Query<(&EntityInstance, &Player)>,
) {
    if input_res.pressed(KeyCode::P) {
        for (entity_instance, player) in &mut query {
            dbg!("{:?}", &entity_instance);
            dbg!("{:?}", &player);
        }
    }
}
