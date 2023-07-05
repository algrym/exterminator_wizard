use std::collections::HashSet;

use bevy::{
    diagnostic::{
        FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};
use bevy::utils::tracing::Instrument;
use bevy_ecs_ldtk::prelude::*;

//  egrep '\t"levelIid"' assets/exterminator_wizard.ldtk | awk '{ print $2 }'
const LEVEL_IIDS: [&str; 4] = [
    "a316bd80-66b0-11ec-9cd7-c50cdc9d2cc4",
    "a317cef0-66b0-11ec-9cd7-dd2f249c8c8b",
    "a315ac10-66b0-11ec-9cd7-99f223ad6ade",
    "a315ac10-66b0-11ec-9cd7-99f223ad6ade",
];

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>) {
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
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .run();
}
