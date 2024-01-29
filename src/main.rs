// main.rs
// github.com/algrym/exterminator_wizard

use bevy::diagnostic::{
    FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy::{
    core_pipeline::{
        bloom::BloomSettings, clear_color::ClearColorConfig, tonemapping::Tonemapping,
    },
    input::common_conditions::input_toggle_active,
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

pub use components::*;

use crate::constants::*;

mod components;
mod constants;
mod map;
mod player;
mod spell_fire;
mod util;

/// This function is the entry point of the "Exterminator Wizard" game.
fn main() {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

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
                .set(RenderPlugin { wgpu_settings })
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            LdtkPlugin,
            PlayerPlugin,
            SpellFirePlugin,
            HanabiPlugin,
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
    camera.camera.hdr = true;
    camera.tonemapping = Tonemapping::default();

    info!("spawn {:?}", camera.camera);
    commands.spawn((camera, BloomSettings::default()));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(MAP_FILENAME),
        ..Default::default()
    });
}
