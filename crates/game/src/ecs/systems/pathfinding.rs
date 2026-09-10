use std::collections::{HashMap, HashSet, VecDeque};

use glam::Vec2;
use hecs::{Entity, World};

use crate::ecs::{
    entities::{player::Player, tile::Tile, zombie::Zombie},
    transform::{Position, Velocity},
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Vec2> for GridPos {
    fn from(value: Vec2) -> Self {
        Self {
            x: (value.x / 8.0).floor() as i32,
            y: (value.y / 8.0).floor() as i32,
        }
    }
}

impl From<&Position> for GridPos {
    fn from(value: &Position) -> Self {
        Self {
            x: (value.x / 8.0).floor() as i32,
            y: (value.y / 8.0).floor() as i32,
        }
    }
}

impl From<Position> for GridPos {
    fn from(value: Position) -> Self {
        Self {
            x: (value.x / 8.0).floor() as i32,
            y: (value.y / 8.0).floor() as i32,
        }
    }
}

fn get_closest_player_pos(world: &World, pos: &Position) -> Option<Position> {
    let pos = pos.vec2();

    world
        .query::<&Position>()
        .with::<&Player>()
        .iter()
        .min_by_key(|player_pos| pos.distance_squared(player_pos.vec2()) as i64)
        .copied()
}

fn outside(pos: &GridPos, constraints: &(GridPos, GridPos)) -> bool {
    pos.x < constraints.0.x
        || pos.x > constraints.1.x
        || pos.y < constraints.0.y
        || pos.y > constraints.1.y
}

fn get_neighbor_pos(current: &GridPos, constraints: &(GridPos, GridPos)) -> Option<[GridPos; 4]> {
    if outside(current, constraints) {
        None
    } else {
        Some([
            GridPos::new(current.x - 1, current.y),
            GridPos::new(current.x + 1, current.y),
            GridPos::new(current.x, current.y - 1),
            GridPos::new(current.x, current.y + 1),
        ])
    }
}

fn reconstruct_path(
    came_from: &HashMap<GridPos, GridPos>,
    start: GridPos,
    target: GridPos,
) -> Vec<GridPos> {
    let mut path = Vec::new();
    let mut current = target;
    path.push(current);
    while current != start {
        current = came_from[&current];
        path.push(current);
    }
    path.reverse();
    path
}

fn bfs_path_finding(
    start: GridPos,
    target: GridPos,
    tiles: &HashSet<GridPos>,
    level_constraints: (GridPos, GridPos),
) -> Vec<GridPos> {
    let mut queue: VecDeque<GridPos> = VecDeque::from([start]);
    let mut visited: HashSet<GridPos> = HashSet::from([start]);
    let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
    if start == target {
        return Vec::new();
    }

    while let Some(current) = queue.pop_front() {
        if current == target {
            println!("visited {}", visited.iter().len());
            return reconstruct_path(&came_from, start, target);
        }
        let neighbors = get_neighbor_pos(&current, &level_constraints);
        if let Some(neighbors) = neighbors {
            for neighbor in neighbors {
                if !tiles.contains(&neighbor) && visited.insert(neighbor) {
                    queue.push_back(neighbor);
                    came_from.insert(neighbor, current);
                }
            }
        }
    }

    Vec::new()
}

pub fn target_did_not_update(
    zombie_paths: &HashMap<Entity, Vec<GridPos>>,
    e: Entity,
    target: &GridPos,
) -> bool {
    if let Some(path) = zombie_paths.get(&e) {
        if let Some(prev_target) = path.last() {
            if target == prev_target {
                return true;
            }
        }
    }

    false
}

pub fn zombie_pathfinding_system(
    world: &mut World,
    tile_grid: &HashSet<GridPos>,
    level_constraints: &(Vec2, Vec2),
    zombie_paths: &mut HashMap<Entity, Vec<GridPos>>,
) {
    let level_constraints: (GridPos, GridPos) =
        (level_constraints.0.into(), level_constraints.1.into());
    for (e, z_pos, vel) in world
        .query::<(Entity, &Position, &mut Velocity)>()
        .with::<&Zombie>()
        .iter()
    {
        if let Some(closest) = get_closest_player_pos(world, z_pos) {
            let target: GridPos = closest.into();
            if target_did_not_update(zombie_paths, e, &target) {
                continue;
            }

            let start: GridPos = z_pos.into();

            let path = bfs_path_finding(start, target, &tile_grid, level_constraints);
            zombie_paths.insert(e, path);
        } else {
            vel.x = 0.0;
            vel.y = 0.0;
        }
    }
}

pub fn build_tile_grid(world: &mut World) -> HashSet<GridPos> {
    world
        .query_mut::<&Position>()
        .with::<&Tile>()
        .into_iter()
        .map(|pos| pos.into())
        .collect()
}
