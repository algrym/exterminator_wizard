use std::collections::HashSet;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::*;
use crate::constants::*;

/// This plugin is responsible for handling map-related functionalities
/// in the game, including processing and caching wall locations.
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<WallBundle>(1)
            .init_resource::<LevelWalls>()
            .add_systems(
                Update,
                (setup_wall_colliders, cache_wall_locations, display_events),
            );
    }
}

/// This plugin is responsible for handling map-related functionalities
/// in the game, including processing and caching wall locations.
#[derive(Default, Resource)]
pub struct LevelWalls {
    wall_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelWalls {
    /// Checks if the given grid coordinates are within a wall.
    ///
    /// # Arguments
    /// * `grid_coords` - The grid coordinates to check.
    ///
    /// # Returns
    /// `true` if the coordinates are within a wall, `false` otherwise.
    pub fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}

/// Caches the locations of walls whenever a level is spawned.
/// This function listens for `LevelEvent::Spawned` events and updates
/// the `LevelWalls` resource with the wall locations for the current level.
fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut level_events: EventReader<LevelEvent>,
    walls: Query<&GridCoords, With<Wall>>,
    ldtk_project_entities: Query<&Handle<LdtkAsset>>,
    ldtk_project_assets: Res<Assets<LdtkAsset>>,
) {
    for level_event in level_events.iter() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single())
                .expect("ERROR: LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_level(&LevelSelection::Iid(level_iid.to_string()))
                .expect("ERROR: spawned level should exist in project");

            let wall_locations = walls.iter().copied().collect();

            let new_level_walls = LevelWalls {
                wall_locations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_walls = new_level_walls;
        }
    }
}

/// Sets up collision components for newly added wall entities.
///
/// This system is designed to run for each entity that has a `Wall` component,
/// but not a `Collider` component. It triggers only when a `Wall` component is newly added
/// to an entity. The system adds a `Collider` component to these entities to handle
/// physical interactions in the game world. Additionally, a `RigidBody::Fixed` component
/// is added to ensure that the walls are stationary and do not move in response to collisions.
///
/// The `Collider` is a cuboid with dimensions based on the wall sprite's width and height,
/// providing an accurate collision area that matches the wall's visual representation.
///
/// # Arguments
/// * `commands` - Provides the functionality to perform various operations on entities,
///   such as adding or removing components.
/// * `query` - Query that selects wall entities requiring collider components.
///
#[allow(clippy::type_complexity)]
fn _naive_setup_wall_colliders(
    mut commands: Commands,
    query: Query<Entity, (With<Wall>, Without<Collider>, Added<Wall>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(Collider::cuboid(
                WALL_SPRITE_WIDTH / 2.0,
                WALL_SPRITE_HEIGHT / 2.0,
            ))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Ccd::enabled())
            .insert(Name::new(format!("Wall {:?}", entity)))
            .insert(RigidBody::Fixed);
    }
    if query.iter().count() > 0 {
        info!("built {} colliders via naïve method", query.iter().count());
    }
}

pub fn setup_wall_colliders(
    mut _commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct _Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        // check each tile and join it with its neighbor if they match
        // this will result in a list of plates, which are wide walls that are 1 tile tall
        let mut plates: HashSet<Plate> = HashSet::new();
        for (&level_entity, wall_locations) in level_to_wall_locations.iter() {
            let mut level_width = 0;
            let mut level_height = 0;
            for wall_location in wall_locations.iter() {
                level_width = level_width.max(wall_location.x);
                level_height = level_height.max(wall_location.y);
            }

            for y in 0..=level_height {
                let mut left = None;
                let mut right = None;
                for x in 0..=level_width {
                    let grid_coords = GridCoords::new(x, y);
                    if wall_locations.contains(&grid_coords) {
                        if left.is_none() {
                            left = Some(x);
                        }
                        right = Some(x);
                    } else {
                        if let (Some(left), Some(right)) = (left, right) {
                            plates.insert(Plate { left, right });
                        }
                        left = None;
                        right = None;
                    }
                }
                if let (Some(left), Some(right)) = (left, right) {
                    plates.insert(Plate { left, right });
                }
            }
            // log the count of colliders vs original
            info!(
                "built {} (down from {}={:.2}%) colliders via plate method",
                plates.len() * 2,
                wall_locations.len(),
                ((plates.len() as f32 * 2.0) / wall_locations.len() as f32) * 100.0
            );

            // Placeholder so we don't forget
            info!(
                "TODO: placeholder level_entity={:?} ({},{})",
                level_entity, WALL_SPRITE_WIDTH, WALL_SPRITE_HEIGHT
            );
        }
    }
}

/* A system that displays the events. */
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_wall() {
        let mut level_walls = LevelWalls {
            wall_locations: HashSet::new(),
            level_width: 10,
            level_height: 10,
        };
        level_walls.wall_locations.insert(GridCoords::new(5, 5));

        assert!(!level_walls.in_wall(&GridCoords::new(1, 1))); // Inside the level and not a wall
        assert!(level_walls.in_wall(&GridCoords::new(5, 5))); // Wall location
        assert!(level_walls.in_wall(&GridCoords::new(-1, 0))); // Outside the level boundaries
        assert!(level_walls.in_wall(&GridCoords::new(10, 10))); // Outside the level boundaries
    }
}
