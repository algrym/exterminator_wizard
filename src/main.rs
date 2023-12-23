use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};

use player::PlayerPlugin;

mod player;

// What is the filename of the map to load?
const MAP_FILENAME: &str = "map.ldtk";

const CAMERA_SCALE: f32 = 0.5;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            LdtkPlugin,
            PanCamPlugin,
            PlayerPlugin,
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_scale(Vec3::splat(CAMERA_SCALE)),
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            ..default()
        })
        .insert(PanCam::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(MAP_FILENAME),
        ..Default::default()
    });
}
