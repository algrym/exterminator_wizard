// main.rs
// github.com/algrym/exterminator_wizard

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::diagnostic::{
    FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

pub use components::*;

use crate::constants::*;

mod components;
mod constants;
mod map;
mod player;
mod util;

/// This function is the entry point of the "Exterminator Wizard" game.
fn main() {
    let primary_window = Window {
        title: format!(
            "Exterminator Wizard v{} - ajw@ajw.io",
            env!("CARGO_PKG_VERSION")
        ),
        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
        resizable: false,
        ..Default::default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            LdtkPlugin,
            PlayerPlugin,
            MapPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(GRID_SIZE as f32),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            SystemInformationDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(LevelSelection::default())
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .run();
}

/// This function initializes the camera and spawns the LDtk world.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = CAMERA_SCALE;
    camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::BLACK);

    info!("spawn camera@{:?}", camera.transform.translation);
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(MAP_FILENAME),
        ..Default::default()
    });
}
