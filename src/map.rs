use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::components::*;
use crate::constants::*;

/// This plugin is responsible for handling map-related functionalities
/// in the game, including processing and caching wall locations.
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<WallBundle>(1)
            .init_resource::<LevelWalls>()
            .add_systems(Update, cache_wall_locations);
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
