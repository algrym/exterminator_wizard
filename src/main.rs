//! In this example we generate a new texture atlas (sprite sheet) from a folder containing
//! individual sprites.

use std::vec::Vec;

use bevy::{
    asset::LoadState,
    diagnostic::{
        FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};

// How long should we pause between player frames?
const PLAYER_ANIMATION_DURATION: f32 = 0.25;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

#[derive(Component)]
struct Player {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct SpriteState {
    facing: Facing,
    animation_index: usize,
}

#[derive(Resource, Default)]
struct EwSpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Component)]
struct SpriteAnimationIndices {
    back: Vec<usize>,
    front: Vec<usize>,
    left: Vec<usize>,
    right: Vec<usize>,
}

impl SpriteAnimationIndices {
    pub fn new(prefix: &str, asset_server: Res<AssetServer>, texture_atlas: TextureAtlas) -> Self {
        let sprite_name_to_index = |sprite_name: String| -> usize {
            texture_atlas
                .get_texture_index(&asset_server.get_handle(sprite_name))
                .unwrap_or_default()
        };

        SpriteAnimationIndices {
            back: vec![
                sprite_name_to_index(format!("{prefix}_bk1.png")),
                sprite_name_to_index(format!("{prefix}_bk2.png")),
            ],
            front: vec![
                sprite_name_to_index(format!("{prefix}_fr1.png")),
                sprite_name_to_index(format!("{prefix}_fr2.png")),
            ],
            left: vec![
                sprite_name_to_index(format!("{prefix}_lf1.png")),
                sprite_name_to_index(format!("{prefix}_lf2.png")),
            ],
            right: vec![
                sprite_name_to_index(format!("{prefix}_rt1.png")),
                sprite_name_to_index(format!("{prefix}_rt2.png")),
            ],
        }
    }
}

fn animate_sprite(
    mut query: Query<(
        &SpriteAnimationIndices,
        &AnimationTimer,
        &mut TextureAtlasSprite,
        &mut SpriteState,
    )>,
) {
    for (indices, timer, mut sprite, mut sprite_state) in &mut query {
        match sprite_state.facing {
            Facing::Down => {
                if timer.just_finished() {
                    sprite_state.animation_index += 1;
                    if sprite_state.animation_index >= indices.front.len() {
                        sprite_state.animation_index = 0;
                    }
                }
                sprite.index = indices.front[sprite_state.animation_index];
            }
            Facing::Up => {
                if timer.just_finished() {
                    sprite_state.animation_index += 1;
                    if sprite_state.animation_index >= indices.back.len() {
                        sprite_state.animation_index = 0;
                    }
                }
                sprite.index = indices.back[sprite_state.animation_index];
            }
            Facing::Left => {
                if timer.just_finished() {
                    sprite_state.animation_index += 1;
                    if sprite_state.animation_index >= indices.left.len() {
                        sprite_state.animation_index = 0;
                    }
                }
                sprite.index = indices.left[sprite_state.animation_index];
            }
            Facing::Right => {
                if timer.just_finished() {
                    sprite_state.animation_index += 1;
                    if sprite_state.animation_index >= indices.right.len() {
                        sprite_state.animation_index = 0;
                    }
                }
                sprite.index = indices.right[sprite_state.animation_index];
            }
        }
    }
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &Player,
        &mut SpriteState,
        &mut AnimationTimer,
        &mut Transform,
    )>,
) {
    for (_player, mut player_facing, mut timer, mut player_transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            player_transform.translation.x -= 1.0;
            player_facing.facing = Facing::Left;
            timer.tick(time.delta());
        }

        if keyboard_input.pressed(KeyCode::Right) {
            player_transform.translation.x += 1.0;
            player_facing.facing = Facing::Right;
            timer.tick(time.delta());
        }

        if keyboard_input.pressed(KeyCode::Up) {
            player_transform.translation.y += 1.0;
            player_facing.facing = Facing::Up;
            timer.tick(time.delta());
        }

        if keyboard_input.pressed(KeyCode::Down) {
            player_transform.translation.y -= 1.0;
            player_facing.facing = Facing::Down;
            timer.tick(time.delta());
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn load_textures(mut ew_sprite_handles: ResMut<EwSpriteHandles>, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder
    ew_sprite_handles.handles = asset_server.load_folder("sprites").unwrap();
}

fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    ew_sprite_handles: ResMut<EwSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    if let LoadState::Loaded = asset_server
        .get_group_load_state(ew_sprite_handles.handles.iter().map(|handle| handle.id()))
    {
        next_state.set(AppState::Finished);
    }
}

fn setup(
    mut commands: Commands,
    ew_sprite_handles: Res<EwSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // Build a `TextureAtlas` using the individual sprites
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &ew_sprite_handles.handles {
        let handle = handle.typed_weak();
        let Some(texture) = textures.get(&handle) else {
            warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(handle));
            continue;
        };

        texture_atlas_builder.add_texture(handle, texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let player_indices =
        SpriteAnimationIndices::new("sprites/amg1", asset_server, texture_atlas.clone());
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());

    // draw a sprite from the atlas
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(150.0, 0.0, 0.0),
                scale: Vec3::splat(3.0),
                ..default()
            },
            sprite: TextureAtlasSprite::new(player_indices.front[0]),
            texture_atlas: atlas_handle,
            ..default()
        },
        Player {},
        SpriteState {
            facing: Facing::Down,
            animation_index: 0,
        },
        player_indices,
        AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_DURATION,
            TimerMode::Repeating,
        )),
    ));
}

fn main() {
    App::new()
        .init_resource::<EwSpriteHandles>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(SystemInformationDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::Setup), load_textures)
        .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
        .add_systems(Update, animate_sprite)
        .add_systems(Update, player_movement_system)
        .add_systems(OnEnter(AppState::Finished), setup)
        .run();
}
