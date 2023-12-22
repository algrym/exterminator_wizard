use bevy::prelude::*;

const SPRITE_SIZE: f32 = 16.0;
const SPRITE_SHEET_SIZE: usize = 32;

const PLAYER_SPRITE_FIRST_INDEX: usize = 104;
const PLAYER_SPRITE_LAST_INDEX: usize = 112;

const ANIMATION_DELAY_SECONDS: f32 = 0.1;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AnimationTimer(Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, animate_player);
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("0x72_DungeonTilesetII_v1.6.png");
    // Define the coordinates and sizes for each frame of the player's animation
    // ...

    let texture_atlas =
        // Adjust the grid size and count based on your sprite sheet
        TextureAtlas::from_grid(texture_handle,
                                Vec2::new(SPRITE_SIZE, SPRITE_SIZE),
                                SPRITE_SHEET_SIZE, SPRITE_SHEET_SIZE,
                                None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            ANIMATION_DELAY_SECONDS,
            TimerMode::Repeating,
        )))
        .insert(Player);
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite), With<Player>>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
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
