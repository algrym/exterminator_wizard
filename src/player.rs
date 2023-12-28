use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::components::*;
use crate::constants::*;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, (move_player, animate_player));
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load the sprite sheet
    let texture_handle = asset_server.load("0x72_DungeonTilesetII_v1.6.png");

    // Break the sprite sheet down into a texture atlas (indexed grid)
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT),
        SPRITE_SHEET_SIZE,
        SPRITE_SHEET_SIZE,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Save the player sprite
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            // Shift the sprite a half-tile horizontally to make it line up
            transform: Transform::from_xyz(SPRITE_WIDTH / 2.0, 0.0, PLAYER_SPRITE_Z),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            ANIMATION_DELAY_SECONDS,
            TimerMode::Repeating,
        )))
        .insert(Name::new("Player"))
        .insert(Player);
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite), With<Player>>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // Keep the sprite index in bounds,
            //   incrementing if possible.
            sprite.index = if (sprite.index >= PLAYER_SPRITE_LAST_INDEX)
                || (sprite.index < PLAYER_SPRITE_FIRST_INDEX)
            {
                PLAYER_SPRITE_FIRST_INDEX
            } else {
                sprite.index + 1
            };
        }
    }
}

fn move_player(
    mut characters: Query<(&mut Transform, &mut TextureAtlasSprite), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), Without<Player>>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let window_size = Vec2::new(window.width(), window.height());

    for (mut transform, mut sprite) in characters.iter_mut() {
        let speed = PLAYER_SPRITE_SPEED * time.delta_seconds();

        let mut move_vec = Vec2::ZERO;

        if input.pressed(KeyCode::W) {
            move_vec.y += speed;
        }
        if input.pressed(KeyCode::S) {
            move_vec.y -= speed;
        }
        if input.pressed(KeyCode::A) {
            move_vec.x -= speed;
            sprite.flip_x = true;
        }
        if input.pressed(KeyCode::D) {
            move_vec.x += speed;
            sprite.flip_x = false;
        }

        transform.translation += move_vec.extend(0.0);

        let (_orthographic_projection, mut camera_transform) = camera_query.single_mut();
        let player_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let camera_pos = Vec2::new(
            camera_transform.translation.x,
            camera_transform.translation.y,
        );

        debug!(
            "player@{:?} {:?} camera@{:?} window@{:?}",
            player_pos,
            camera_pos.distance(player_pos),
            camera_pos,
            window_size,
        );

        // If the player x is more than a quarter of the (scaled)
        //   window size to the camera, move the camera.
        // TODO: camera position needs to reset on window resize event
        if (camera_pos.x - player_pos.x).abs() > (window_size.x * CAMERA_SCALE_QUARTER) {
            camera_transform.translation.x += move_vec.x;
        }

        if (camera_pos.y - player_pos.y).abs() > (window_size.y * CAMERA_SCALE_QUARTER) {
            camera_transform.translation.y += move_vec.y;
        }
    }
}
