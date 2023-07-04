use bevy::{
    diagnostic::{
        FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};
use bevy_ecs_ldtk::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("exterminator_wizard.ldtk"),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(LdtkPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(SystemInformationDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .run();
}
