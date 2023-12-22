use bevy::prelude::*;

const SPRITE_HEIGHT: f32 = 32.0;
const SPRITE_WIDTH: f32 = 16.0;
const SPRITE_SHEET_SIZE: usize = 32;

const PLAYER_SPRITE_FIRST_INDEX: usize = 136;
const PLAYER_SPRITE_LAST_INDEX: usize = 144;

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
    // Load the sprite sheet
    let texture_handle = asset_server.load("0x72_DungeonTilesetII_v1.6.png");

    // Break the sprite sheet down into a texture atlas (indexed grid)
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle,
                                Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT),
                                SPRITE_SHEET_SIZE, SPRITE_SHEET_SIZE,
                                None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Save the player sprite
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            // Shift the sprite a half-tile horizontally to make it line up
            transform: Transform::from_xyz(SPRITE_WIDTH / 2.0, 0.0, 0.0),
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
