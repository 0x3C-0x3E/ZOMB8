use std::collections::{HashMap, HashSet, VecDeque};

use hecs::World;

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

impl From<&Position> for GridPos {
    fn from(value: &Position) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

fn get_closest_player_pos(world: &World, pos: &Position) -> Option<Position> {
    let mut closest: Option<Position> = None;
    for player_pos in world.query::<&Position>().with::<&Player>().iter() {
        if let Some(prev_closest) = closest {
            if pos.vec2().distance_squared(prev_closest.vec2())
                > pos.vec2().distance_squared(player_pos.vec2())
            {
                closest = Some(*player_pos);
            }
        } else {
            closest = Some(*player_pos);
        }
    }

    closest
}

fn get_neighbor_pos(current: &GridPos) -> [GridPos; 4] {
    [
        GridPos::new(current.x - 8, current.y),
        GridPos::new(current.x + 8, current.y),
        GridPos::new(current.x, current.y - 8),
        GridPos::new(current.x, current.y + 8),
    ]
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

fn bfs_path_finding(start: GridPos, target: GridPos, tiles: &HashSet<GridPos>) -> Vec<GridPos> {
    let mut queue: VecDeque<GridPos> = VecDeque::from([start]);
    let mut visited: HashSet<GridPos> = HashSet::new();
    let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
    if start == target {
        return Vec::new();
    }

    while let Some(current) = queue.pop_front() {
        if current == target {
            return reconstruct_path(&came_from, start, target);
        }
        let neighbors = get_neighbor_pos(&current);
        for neighbor in neighbors {
            if !tiles.contains(&neighbor) && visited.insert(neighbor) {
                queue.push_back(neighbor);
                came_from.insert(neighbor, current);
            }
        }
    }

    Vec::new()
}

pub fn zombie_pathfinding_system(world: &mut World) {
    let tiles = build_tile_hash_set(world);

    for (z_pos, vel) in world
        .query::<(&Position, &mut Velocity)>()
        .with::<&Zombie>()
        .iter()
    {
        if let Some(closest) = get_closest_player_pos(world, z_pos) {
            let target: GridPos = (&closest).into();
            let start: GridPos = z_pos.into();
            let path = bfs_path_finding(start, target, &tiles);
        } else {
            vel.x = 0.0;
            vel.y = 0.0;
        }
    }
}

fn build_tile_hash_set(world: &mut World) -> HashSet<GridPos> {
    world
        .query_mut::<&Position>()
        .with::<&Tile>()
        .into_iter()
        .map(|pos| pos.into())
        .collect()
}
