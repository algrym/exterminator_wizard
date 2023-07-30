use std::vec::Vec;

use bevy::{
    asset::LoadState,
    diagnostic::{
        FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod helpers;

// What sprites should we use for the player?
const PLAYER_SPRITE_NAME: &str = "sprites/amg1";

// What is the filename of the map to load?
const MAP_FILENAME: &str = "map.tmx";

// How long should we pause between player frames?
const PLAYER_ANIMATION_DURATION: f32 = 0.25;

// How fast should the player move per tick?
const PLAYER_MOVEMENT_SPEED: f32 = 100.0;

// How large should sprites be scaled to?
const SPRITE_SCALE: f32 = 1.5;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    LoadingStart,
    LoadingFinished,
}

#[derive(Reflect, Component)]
struct Player {}

#[derive(Reflect, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
enum Facing {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

#[derive(Default, Reflect, Component)]
struct SpriteState {
    facing: Facing,
    animation_index: usize,
}

#[derive(Resource, Default)]
struct EwSpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Reflect, Debug, Component)]
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
    mut query: Query<(&mut SpriteState, &mut AnimationTimer, &mut Transform), With<Player>>,
) {
    for (mut player_facing, mut player_animation_timer, mut player_transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            player_transform.translation.x -= PLAYER_MOVEMENT_SPEED * time.delta_seconds();
            player_facing.facing = Facing::Left;
            player_animation_timer.tick(time.delta());
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            player_transform.translation.x += PLAYER_MOVEMENT_SPEED * time.delta_seconds();
            player_facing.facing = Facing::Right;
            player_animation_timer.tick(time.delta());
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            player_transform.translation.y += PLAYER_MOVEMENT_SPEED * time.delta_seconds();
            player_facing.facing = Facing::Up;
            player_animation_timer.tick(time.delta());
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            player_transform.translation.y -= PLAYER_MOVEMENT_SPEED * time.delta_seconds();
            player_facing.facing = Facing::Down;
            player_animation_timer.tick(time.delta());
        }
    }
}

#[derive(Reflect, Component, Deref, DerefMut)]
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
        next_state.set(AppState::LoadingFinished);
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

    // setup the map
    let map_handle: Handle<helpers::tiled::TiledMap> = asset_server.load(MAP_FILENAME);

    commands.spawn(helpers::tiled::TiledMapBundle {
        tiled_map: map_handle,
        transform: Transform {
            scale: Vec3::splat(SPRITE_SCALE),
            ..default()
        },
        ..Default::default()
    });

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let player_indices =
        SpriteAnimationIndices::new(PLAYER_SPRITE_NAME, asset_server, texture_atlas.clone());
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());

    // setup player sprite from the atlas
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::splat(SPRITE_SCALE),
                ..default()
            },
            sprite: TextureAtlasSprite::new(player_indices.front[0]),
            texture_atlas: atlas_handle,
            ..default()
        },
        Player {},
        Name::new("Player"),
        SpriteState {
            ..Default::default()
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
        // bevy_inspector_egui wants local objects registered
        .register_type::<AnimationTimer>()
        .register_type::<Facing>()
        .register_type::<Player>()
        .register_type::<SpriteState>()
        .register_type::<SpriteAnimationIndices>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // prevents blurry sprites
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false, // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                        title: format!(
                            "Exterminator Wizard v{} - ajw@ajw.io",
                            env!("CARGO_PKG_VERSION")
                        ),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(SystemInformationDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::LoadingStart), load_textures)
        .add_systems(Update, check_textures.run_if(in_state(AppState::LoadingStart)))
        .add_systems(Update, animate_sprite)
        .add_systems(Update, player_movement_system)
        .add_systems(OnEnter(AppState::LoadingFinished), setup)
        .add_plugins(TilemapPlugin)
        .add_plugins(helpers::tiled::TiledMapPlugin)
        .run();
}
