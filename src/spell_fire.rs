// spell_fire.rs

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy_rapier2d::prelude::*;

use crate::components::*;
use crate::constants::*;

impl Plugin for SpellFirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animate_spell_fire,
                setup_spell_fire_animation,
                setup_spell_fire_collision,
                spawn_spell_fire_from_input,
                dbg_spell_fire.run_if(on_timer(Duration::from_secs(1))),
            ),
        );
    }
}

/// Sets up the animation component for any added spellfire entities.
///
/// This system runs for each entity that has a `spellfire` component but not an `Animation` component.
/// It is triggered only when a `spellfire` component is newly added to an entity.
/// The system adds an `Animation` component with predefined frames to these entities.
///
/// # Arguments
/// * `commands` - Used to perform commands on entities such as adding components.
/// * `query` - Query to select entities that are spellfires and require an animation component.
///
#[allow(clippy::type_complexity)]
fn setup_spell_fire_animation(
    mut commands: Commands,
    query: Query<Entity, (With<SpellFire>, Without<Animation>, Added<SpellFire>)>,
) {
    for entity in query.iter() {
        info!("Adding animation to spellfire entity: {:?}", entity);
        commands.entity(entity).insert(Animation {
            frames: SPELL_FIRE_SPRITE_FRAMES.to_vec(),
            ..default()
        });
    }
}

fn animate_spell_fire(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite), With<SpellFire>>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.finished() {
            let next_frame = (animation
                .frames
                .iter()
                .position(|&frame| frame == sprite.index)
                .unwrap_or(0)
                + 1)
                % animation.frames.len();
            sprite.index = animation.frames[next_frame];
        }
    }
}

#[allow(clippy::type_complexity)]
fn setup_spell_fire_collision(
    mut commands: Commands,
    query: Query<Entity, (With<SpellFire>, Without<Collider>, Added<SpellFire>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(Collider::cuboid(
                SPELL_FIRE_SPRITE_WIDTH / 2.0,
                SPELL_FIRE_SPRITE_HEIGHT / 2.0,
            ))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(RigidBody::Dynamic)
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled())
            .insert(Name::new(format!("Spell_Fire {:?}", entity)));
    }
}

fn spawn_spell_fire(
    commands: &mut Commands,
    transform_player: &Transform,
    asset_server: &AssetServer,
    impulse: Vec2,
) {
    info!("spawning 🔥impulse: {:?}", impulse);
    let mut transform_spell_fire = *transform_player;
    transform_spell_fire.translation.z += 1.0;

    commands
        .spawn(SpellFire)
        .insert(Animation {
            frames: SPELL_FIRE_SPRITE_FRAMES.to_vec(),
            timer: Timer::from_seconds(SPRITE_ANIMATION_SPEED, TimerMode::Repeating),
        })
        .insert(transform_spell_fire)
        .insert(SpriteSheetBundle {
            // TODO: spell_fire needs a sprite_sheet_bundle. This ain't working.
            texture_atlas: asset_server.load(SPELL_FIRE_SPRITE_SHEET),
            sprite: TextureAtlasSprite::new(SPELL_FIRE_SPRITE_FRAMES[0]),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        })
        .insert(ExternalImpulse {
            impulse,
            torque_impulse: 0.0,
        });
}

/// When the player presses an arrow key, shoot a Spell_Fire in that direction.
fn spawn_spell_fire_from_input(
    mut commands: Commands,
    input_res: Res<Input<KeyCode>>,
    query: Query<&mut Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    for transform in query.iter() {
        if input_res.just_pressed(KeyCode::Up) {
            spawn_spell_fire(
                &mut commands,
                transform,
                &asset_server,
                Vec2::new(0.0, SPELL_FIRE_SPEED),
            );
        }
        if input_res.just_pressed(KeyCode::Down) {
            spawn_spell_fire(
                &mut commands,
                transform,
                &asset_server,
                Vec2::new(0.0, -SPELL_FIRE_SPEED),
            );
        }
        if input_res.just_pressed(KeyCode::Left) {
            spawn_spell_fire(
                &mut commands,
                transform,
                &asset_server,
                Vec2::new(-SPELL_FIRE_SPEED, 0.0),
            );
        }
        if input_res.just_pressed(KeyCode::Right) {
            spawn_spell_fire(
                &mut commands,
                transform,
                &asset_server,
                Vec2::new(SPELL_FIRE_SPEED, 0.0),
            );
        }
    }
}

fn dbg_spell_fire(query: Query<(&Transform, &Collider, &SpellFire)>) {
    for (transform, collider, spell_fire) in query.iter() {
        info!(
            "🔥dbg_spell_fire: {:?} {:?} {:?}",
            transform, collider, spell_fire
        );
    }
}
