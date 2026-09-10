use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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
) -> VecDeque<GridPos> {
    let mut path = VecDeque::new();
    let mut current = target;
    path.push_back(current);
    while current != start {
        current = came_from[&current];
        path.push_back(current);
    }
    path.make_contiguous().reverse();
    path
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct QueueNode {
    pub cost: u32,
    pub pos: GridPos,
}

impl QueueNode {
    fn new(cost: u32, pos: GridPos) -> Self {
        Self { cost, pos }
    }
}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn get_manhatten_distance(start: &GridPos, target: &GridPos) -> u32 {
    (start.x - target.x).unsigned_abs() + (start.y - target.y).unsigned_abs()
}

fn bfs_path_finding(
    start: GridPos,
    target: GridPos,
    tiles: &HashSet<GridPos>,
    level_constraints: (GridPos, GridPos),
) -> VecDeque<GridPos> {
    let mut open: BinaryHeap<QueueNode> = BinaryHeap::from([QueueNode::new(
        get_manhatten_distance(&start, &target),
        start,
    )]);

    let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();

    let mut g_score = HashMap::from([(start, 0u32)]);

    if start == target {
        return VecDeque::new();
    }

    while let Some(current) = open.pop() {
        if current.pos == target {
            return reconstruct_path(&came_from, start, target);
        }

        let current_g = g_score[&current.pos];

        let neighbors = get_neighbor_pos(&current.pos, &level_constraints);
        if let Some(neighbors) = neighbors {
            for neighbor in neighbors {
                if tiles.contains(&neighbor) {
                    continue;
                }

                let tentative_g = current_g + 1;

                if tentative_g < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                    came_from.insert(neighbor, current.pos);
                    g_score.insert(neighbor, tentative_g);
                    let h = get_manhatten_distance(&neighbor, &target);

                    open.push(QueueNode::new(tentative_g + h, neighbor));
                }
            }
        }
    }

    VecDeque::new()
}

pub fn target_did_not_update(
    zombie_paths: &HashMap<Entity, VecDeque<GridPos>>,
    e: Entity,
    target: &GridPos,
) -> bool {
    if let Some(path) = zombie_paths.get(&e) {
        if let Some(prev_target) = path.back() {
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
    zombie_paths: &mut HashMap<Entity, VecDeque<GridPos>>,
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

            let mut path = bfs_path_finding(start, target, &tile_grid, level_constraints);
            path.pop_front();
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
