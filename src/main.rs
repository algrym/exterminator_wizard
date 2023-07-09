use std::collections::HashSet;

use bevy::{
    diagnostic::{
        FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};
use bevy_ecs_ldtk::prelude::*;

pub mod player;

//  egrep '\t"levelIid"' assets/exterminator_wizard.ldtk | awk '{ print $2 }'
const LEVEL_IIDS: [&str; 4] = [
    "a316bd80-66b0-11ec-9cd7-c50cdc9d2cc4",
    "a317cef0-66b0-11ec-9cd7-dd2f249c8c8b",
    "a315ac10-66b0-11ec-9cd7-99f223ad6ade",
    "a315ac10-66b0-11ec-9cd7-99f223ad6ade",
];


#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn setup_player (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale /= 2.0;
    commands.spawn(camera);

    let iids: HashSet<String> = LEVEL_IIDS.into_iter().map(|s| s.to_string()).collect();

    let window = windows.single();
    info!("Window size: {},{}", window.width(), window.height());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("exterminator_wizard.ldtk"),
        level_set: LevelSet { iids },
        transform: Transform::from_xyz(window.width() / -2.0,
                                       window.height() / -2.0,
                                       0.0),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )

        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(SystemInformationDiagnosticsPlugin::default())

        .add_startup_system(setup_map)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugin(LdtkPlugin)

        .add_startup_system(setup_player)
        .add_system(animate_sprite)
        .run();
}
