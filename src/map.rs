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

#[allow(clippy::type_complexity)]
pub fn setup_wall_colliders(
    mut commands: Commands,
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
    #[derive(Clone, Eq, PartialEq, Debug, Default)]
    struct Rect {
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

    let mut grandparent_entity: Option<Entity> = None;
    wall_query.for_each(|(&grid_coords, parent)| {
        // An integer grid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
            grandparent_entity = Some(grandparent.get()); // Store the grandparent entity ID for later
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

            // combine wall tiles into flat "plates" in each individual row
            let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

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

            // combine "plates" into rectangles across multiple rows
            let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
            let mut prev_row: Vec<Plate> = Vec::new();
            let mut wall_rects: Vec<Rect> = Vec::new();

            // an extra empty row so the algorithm "finishes" the rects that touch the top edge
            plate_stack.push(Vec::new());

            for (y, current_row) in plate_stack.into_iter().enumerate() {
                for prev_plate in &prev_row {
                    if !current_row.contains(prev_plate) {
                        // remove the finished rect so that the same plate in the future starts a new rect
                        if let Some(rect) = rect_builder.remove(prev_plate) {
                            wall_rects.push(rect);
                        }
                    }
                }
                for plate in &current_row {
                    rect_builder
                        .entry(plate.clone())
                        .and_modify(|e| e.top += 1)
                        .or_insert(Rect {
                            bottom: y as i32,
                            top: y as i32,
                            left: plate.left,
                            right: plate.right,
                        });
                }
                prev_row = current_row;
            }

            // Placeholder so we don't forget
            info!(
                "TODO: placeholder level_entity={:?} ({},{})",
                level_entity, WALL_SPRITE_WIDTH, WALL_SPRITE_HEIGHT
            );

            // Spawn colliders for every rectangle and add them as children of the level entity, stored in grandparent_entity
            for wall_rect in wall_rects.iter() {
                if let Some(grandparent_id) = grandparent_entity {
                    info!(
                        "new wall_rect={:?} -> entity={:?} id={:?}",
                        wall_rect, grandparent_entity, grandparent_id
                    );

                    commands
                        .entity(grandparent_id)
                        .insert(Collider::cuboid(
                            (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                * GRID_SIZE as f32
                                / 2.,
                            (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                * GRID_SIZE as f32
                                / 2.,
                        ))
                        .insert(RigidBody::Fixed)
                        .insert(Friction::new(1.0))
                        .insert(Transform::from_xyz(
                            (wall_rect.left + wall_rect.right + 1) as f32 * GRID_SIZE as f32 / 2.,
                            (wall_rect.bottom + wall_rect.top + 1) as f32 * GRID_SIZE as f32 / 2.,
                            0.,
                        ));
                }
            }

            // log the count of colliders vs original
            info!(
                "built {} (from {}) colliders via plate method",
                wall_rects.len(),
                wall_locations.len(),
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
