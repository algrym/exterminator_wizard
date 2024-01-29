// spell_fire.rs

use bevy::{
    prelude::*, render::mesh::shape::Cube, time::common_conditions::on_timer, utils::Duration,
};
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::*;
use crate::constants::*;

impl Plugin for SpellFirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                setup_spell_fire_effect,
                // setup_spell_fire_collision,
                spawn_spell_fire_from_input,
                dbg_spell_fire.run_if(on_timer(Duration::from_secs(1))),
            ),
        );
    }
}

fn setup_spell_fire_effect(
    asset_server: Res<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("cloud.png");

    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::splat(1.0));
    gradient.add_key(0.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
    gradient.add_key(0.4, Vec4::new(1.0, 0.0, 0.0, 1.0));
    gradient.add_key(1.0, Vec4::splat(0.0));

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(5.).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(1.).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(2.).expr(),
    };

    effects.add(
        EffectAsset::new(32768, Spawner::rate(1000.0.into()), writer.finish())
            .with_name("spell_fire")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ParticleTextureModifier {
                texture: texture_handle.clone(),
            })
            .render(ColorOverLifetimeModifier { gradient }),
    );
}

#[allow(clippy::type_complexity)]
fn _setup_spell_fire_collision(
    mut commands: Commands,
    query: Query<Entity, (With<SpellFire>, Without<Collider>, Added<SpellFire>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(Collider::cuboid(
                _SPELL_FIRE_SPRITE_WIDTH / 2.0,
                _SPELL_FIRE_SPRITE_HEIGHT / 2.0,
            ))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(RigidBody::Dynamic)
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled())
            .insert(Name::new(format!("Spell_Fire {:?}", entity)));
    }
}

/// When the player presses an arrow key, shoot a Spell_Fire in that direction.
fn spawn_spell_fire_from_input(
    mut commands: Commands,
    input_res: Res<Input<KeyCode>>,
    query: Query<&mut Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for player_transform in query.iter() {
        let impulse = if input_res.just_pressed(KeyCode::Up) {
            Vec2::new(0.0, SPELL_FIRE_SPEED)
        } else if input_res.just_pressed(KeyCode::Down) {
            Vec2::new(0.0, -SPELL_FIRE_SPEED)
        } else if input_res.just_pressed(KeyCode::Left) {
            Vec2::new(-SPELL_FIRE_SPEED, 0.0)
        } else if input_res.just_pressed(KeyCode::Right) {
            Vec2::new(SPELL_FIRE_SPEED, 0.0)
        } else {
            Vec2::ZERO
        };

        if impulse != Vec2::ZERO {
            let texture_handle: Handle<Image> = asset_server.load("cloud.png");
            let spell_transform = Transform::from_translation(Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y,
                player_transform.translation.z + 1.0,
            ));

            let mut gradient = Gradient::new();
            gradient.add_key(0.0, Vec4::splat(1.0));
            gradient.add_key(0.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
            gradient.add_key(0.4, Vec4::new(1.0, 0.0, 0.0, 1.0));
            gradient.add_key(1.0, Vec4::splat(0.0));

            let writer = ExprWriter::new();

            let age = writer.lit(0.).expr();
            let init_age = SetAttributeModifier::new(Attribute::AGE, age);

            let lifetime = writer.lit(5.).expr();
            let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

            let init_pos = SetPositionSphereModifier {
                center: writer.lit(Vec3::ZERO).expr(),
                radius: writer.lit(1.).expr(),
                dimension: ShapeDimension::Volume,
            };

            let init_vel = SetVelocitySphereModifier {
                center: writer.lit(Vec3::ZERO).expr(),
                speed: writer.lit(2.).expr(),
            };

            let effect = effects.add(
                EffectAsset::new(32768, Spawner::rate(1000.0.into()), writer.finish())
                    .with_name("spell_fire")
                    .init(init_pos)
                    .init(init_vel)
                    .init(init_age)
                    .init(init_lifetime)
                    .render(ParticleTextureModifier {
                        texture: texture_handle.clone(),
                    })
                    .render(ColorOverLifetimeModifier { gradient }),
            );

            info!(
                "🔥spawn spell_fire@{:?} impulse@{:?}",
                spell_transform.translation, impulse
            );

            commands
                .spawn(SpellFire)
                .insert(Name::new("spell_fire"))
                .insert(spell_transform)
                .insert(ParticleEffectBundle::new(effect))
                // .insert(ExternalImpulse {
                //     impulse: impulse,
                //     torque_impulse: 0.0,
                // })
                .with_children(|p| {
                    p.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(Cube { size: 1.0 })),
                        material: materials.add(Color::RED.into()),
                        ..Default::default()
                    });
                });
        }
    }
}

fn dbg_spell_fire(query: Query<&Transform, With<SpellFire>>) {
    for transform in query.iter() {
        info!("🔥dbg_spell_fire: {:?}", transform.translation);
    }
}
