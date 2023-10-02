use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::api::loop_detection::detect_loop;
use std::collections::VecDeque;

use super::save_kb::save_bfs_path_to_file;

const BREEZE: char = 'b';
const STENCH: char = 's';
const GLITTER: char = 'g';
const NORMAL: char = 'n';

const WUMPUS_WORLD_SIZE: i32 = 10;

#[derive(Debug, Serialize, Deserialize)]
pub struct CellKnowledge {
    x: usize,
    y: usize,

    pit: bool,
    wumpus: bool,
    gold: bool,

    countBreezeSensedNearby: u64,
    countStenchSensedNearby: u64,
    countGlitterSensedNearby: u64,

    visited: bool,
    visitedCount: u32
}

impl CellKnowledge {
    pub fn new(x: usize, y: usize) -> CellKnowledge {
        CellKnowledge {
            x,
            y,
            pit: true,
            wumpus: true,
            gold: true,

            countBreezeSensedNearby: 0,
            countStenchSensedNearby: 0,
            countGlitterSensedNearby: 0,

            visited: false,
            visitedCount: 0
        }
    }
}

pub fn initialize_knowledge_base(knowledge_base: &mut Vec<Vec<CellKnowledge>>) {
    for i in 0..WUMPUS_WORLD_SIZE {
        let mut row: Vec<CellKnowledge> = Vec::new();
        for j in 0..WUMPUS_WORLD_SIZE {
            row.push(CellKnowledge::new(i as usize, j as usize));
        }
        knowledge_base.push(row);
    }
}

fn remove_stench_from_knowledge_base_at(
    x: i32,
    y: i32,
    knowledge_base: &mut Vec<Vec<CellKnowledge>>,
) {
    if x < 0 || y < 0 || x as i32 >= WUMPUS_WORLD_SIZE || y as i32 >= WUMPUS_WORLD_SIZE {
        return;
    }

    if knowledge_base[x as usize][y as usize].countStenchSensedNearby != 0 {
        knowledge_base[x as usize][y as usize].countStenchSensedNearby -= 1;
    } else {
        knowledge_base[x as usize][y as usize].wumpus = false;
    }
}

fn update_knowledge_base(
    x: i32,
    y: i32,
    perceived_arr: &Vec<char>,
    knowledge_base: &mut Vec<Vec<CellKnowledge>>,
) -> () {
    if x < 0 || y < 0 || x >= WUMPUS_WORLD_SIZE || y >= WUMPUS_WORLD_SIZE {
        return;
    }

    let x: usize = x as usize;
    let y: usize = y as usize;

    if knowledge_base[x][y].visited {return;} // NEW ADDITION

    for &perceived in perceived_arr {
        knowledge_base[x][y].pit &= perceived == BREEZE;
        knowledge_base[x][y].wumpus &= perceived == STENCH;
        knowledge_base[x][y].gold &= perceived == GLITTER;

        if perceived == BREEZE {
            knowledge_base[x][y].countBreezeSensedNearby =
                std::cmp::max(knowledge_base[x][y].countBreezeSensedNearby + 1, 3);
        } else if perceived == STENCH {
            knowledge_base[x][y].countStenchSensedNearby =
                std::cmp::max(knowledge_base[x][y].countStenchSensedNearby + 1, 3);
        } else if perceived == GLITTER {
            knowledge_base[x][y].countGlitterSensedNearby =
                std::cmp::max(knowledge_base[x][y].countGlitterSensedNearby + 1, 3);
        } else if perceived == NORMAL {
            knowledge_base[x][y].countGlitterSensedNearby =
                std::cmp::min(knowledge_base[x][y].countGlitterSensedNearby + 1, 0);
            knowledge_base[x][y].countBreezeSensedNearby =
                std::cmp::min(knowledge_base[x][y].countBreezeSensedNearby + 1, 0);
            knowledge_base[x][y].countStenchSensedNearby =
                std::cmp::min(knowledge_base[x][y].countStenchSensedNearby + 1, 0);
        }
    }
}

fn predicate_glittery_and_safe_path(
    x: i32,
    y: i32,
    knowledge_base: &mut Vec<Vec<CellKnowledge>>,
) -> bool {
    if x < 0 || y < 0 || x >= WUMPUS_WORLD_SIZE || y >= WUMPUS_WORLD_SIZE {
        return false;
    }

    let x: usize = x as usize;
    let y: usize = y as usize;

    if knowledge_base[x][y].countGlitterSensedNearby >= 1
        && knowledge_base[x][y].pit == false
        && knowledge_base[x][y].wumpus == false
    {
        return true;
    }

    return false;
}

fn predicate_safe_unvisited_path(
    x: i32,
    y: i32,
    knowledge_base: &mut Vec<Vec<CellKnowledge>>,
) -> bool {
    if x < 0 || y < 0 || x >= WUMPUS_WORLD_SIZE || y >= WUMPUS_WORLD_SIZE {
        return false;
    }

    let x: usize = x as usize;
    let y: usize = y as usize;

    if knowledge_base[x][y].visited == false
        && knowledge_base[x][y].pit == false
        && knowledge_base[x][y].wumpus == false
    {
        return true;
    }

    return false;
}

fn predicate_throw_arrow(
    x: i32,
    y: i32,
    knowledge_base: &mut Vec<Vec<CellKnowledge>>,
    num_of_arrows: &mut u32,
) -> bool {
    if x < 0 || y < 0 || x >= WUMPUS_WORLD_SIZE || y >= WUMPUS_WORLD_SIZE {
        return false;
    }

    if knowledge_base[x as usize][y as usize].countStenchSensedNearby >= 2
        && knowledge_base[x as usize][y as usize].countBreezeSensedNearby < 2
        && *num_of_arrows > 0
    {
        *num_of_arrows -= 1;

        remove_stench_from_knowledge_base_at(x - 1, y, knowledge_base);
        remove_stench_from_knowledge_base_at(x + 1, y, knowledge_base);
        remove_stench_from_knowledge_base_at(x, y - 1, knowledge_base);
        remove_stench_from_knowledge_base_at(x, y + 1, knowledge_base);

        return true;
    }
    return false;
}

fn exclude_death_paths(
    x: i32,
    y: i32,
    knowledge_base: &mut Vec<Vec<CellKnowledge>>,
) -> Vec<(usize, usize)> {
    let mut less_dangerous_paths: Vec<(usize, usize)> = Vec::new();

    let x: usize = x as usize;
    let y: usize = y as usize;

    if x != 0
        && knowledge_base[x - 1][y].countBreezeSensedNearby < 2
        && knowledge_base[x - 1][y].countStenchSensedNearby < 2
        && knowledge_base[x - 1][y].visited == false
    {
        less_dangerous_paths.push((x - 1, y));
    } else if x + 1 != WUMPUS_WORLD_SIZE as usize
        && knowledge_base[x + 1][y].countBreezeSensedNearby < 2
        && knowledge_base[x + 1][y].countStenchSensedNearby < 2
        && knowledge_base[x + 1][y].visited == false
    {
        less_dangerous_paths.push((x + 1, y));
    } else if y != 0
        && knowledge_base[x][y - 1].countBreezeSensedNearby < 2
        && knowledge_base[x][y - 1].countStenchSensedNearby < 2
        && knowledge_base[x][y - 1].visited == false
    {
        less_dangerous_paths.push((x, y - 1));
    } else if y + 1 != WUMPUS_WORLD_SIZE as usize
        && knowledge_base[x][y + 1].countBreezeSensedNearby < 2
        && knowledge_base[x][y + 1].countStenchSensedNearby < 2
        && knowledge_base[x][y + 1].visited == false
    {
        less_dangerous_paths.push((x, y + 1));
    }

    return less_dangerous_paths;
}
fn find_least_dangerous_location(
    frontier_cells: &Vec<(usize, usize)>,
    knowledge_base: &Vec<Vec<CellKnowledge>>,
) -> Option<(usize, usize)> {
    let mut min_danger = std::u64::MAX; // Change to u64
    let mut least_dangerous_location: Option<(usize, usize)> = None;

    for &(i, j) in frontier_cells {
        let danger_level = (knowledge_base[i][j].countBreezeSensedNearby
            * knowledge_base[i][j].countBreezeSensedNearby)
            + (knowledge_base[i][j].countStenchSensedNearby
                * knowledge_base[i][j].countStenchSensedNearby) +  knowledge_base[i][j].visitedCount as u64;
                ;

        if danger_level < min_danger {
            min_danger = danger_level;
            least_dangerous_location = Some((i, j));
        }
    }

    least_dangerous_location
}

fn find_frontier_cells(knowledge_base: &Vec<Vec<CellKnowledge>>) -> Vec<(usize, usize)> {
    let mut frontier_cells: Vec<(usize, usize)> = Vec::new();
    let directions = [(0, 1), (0, -1), (-1, 0), (1, 0)];

    // Iterate through the knowledge base
    for i in 0..knowledge_base.len() {
        for j in 0..knowledge_base[i].len() {
            let cell = &knowledge_base[i][j];

            if cell.visited {
                let mut has_unexplored_neighbor = false;
                for &(dx, dy) in &directions {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;

                    if x >= 0
                        && y >= 0
                        && (x as usize) < knowledge_base.len()
                        && (y as usize) < knowledge_base[i].len()
                    {
                        if !knowledge_base[x as usize][y as usize].visited {
                            has_unexplored_neighbor = true;
                            break;
                        }
                    }
                }
                if has_unexplored_neighbor {
                    frontier_cells.push((i, j));
                }
            }
        }
    }

    frontier_cells
}

pub fn backtrack(x: i32, y: i32, knowledge_base: &Vec<Vec<CellKnowledge>>) -> bool {
    if x < 0 || y < 0 || x >= WUMPUS_WORLD_SIZE || y >= WUMPUS_WORLD_SIZE {
        return false;
    }

    let x: usize = x as usize;
    let y: usize = y as usize;

    return knowledge_base[x][y].visited;
}

fn find_shortest_path_bfs(
    x: usize,
    y: usize,
    target: (usize, usize),
    knowledge_base: &Vec<Vec<CellKnowledge>>,
) -> Option<Vec<(usize, usize)>> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; knowledge_base[0].len()]; knowledge_base.len()];

    for i in 0..knowledge_base.len() {
        for j in 0..knowledge_base[i].len() {
            if (knowledge_base[i][j].visited == false) {
                visited[knowledge_base[i][j].x][knowledge_base[i][j].y] = true;
            }
        }
    }

    let mut parent = vec![vec![(0, 0); knowledge_base[0].len()]; knowledge_base.len()];

    queue.push_back((x, y));
    visited[x][y] = true;

    while let Some((curr_x, curr_y)) = queue.pop_front() {
        if (curr_x, curr_y) == target {
            let mut path = Vec::new();
            let mut node = target;

            while node != (x, y) {
                path.push(node);
                node = parent[node.0][node.1];
            }

            path.push((x, y));
            path.reverse();
            return Some(path);
        }

        for &(dx, dy) in &[(0, 1), (0, -1), (-1, 0), (1, 0)] {
            let new_x = curr_x as i32 + dx;
            let new_y = curr_y as i32 + dy;

            if new_x >= 0
                && new_y >= 0
                && new_x < knowledge_base.len() as i32
                && new_y < knowledge_base[0].len() as i32
                && !visited[new_x as usize][new_y as usize]
            {
                queue.push_back((new_x as usize, new_y as usize));
                visited[new_x as usize][new_y as usize] = true;
                parent[new_x as usize][new_y as usize] = (curr_x, curr_y);
            }
        }
    }

    None
}

pub fn get_next_move(
    x: i32,
    y: i32,
    perceived: &Vec<char>,
    knowledge_base: &mut Vec<Vec<CellKnowledge>>,
    num_of_arrows: &mut u32,
) -> (i32, i32) {
    // BFS Path file is being emptied. Because the API Endpoint can only know of a BFS path by inspecting the file (since we can't return the path vector from the make_move() function which returns a tuple(x,y)) So we save the BFS path in a file in case a loop is detected. And in the following code, we clear the file.
    let temp: Vec<(usize, usize)> = Vec::new();
    let _ = save_bfs_path_to_file(temp, &String::from("bfs_path.txt"));

    knowledge_base[x as usize][y as usize].visited = true;
    knowledge_base[x as usize][y as usize].visitedCount += 1;

    if !detect_loop() {
        update_knowledge_base(x - 1, y, &perceived, knowledge_base);
        update_knowledge_base(x + 1, y, &perceived, knowledge_base);
        update_knowledge_base(x, y - 1, &perceived, knowledge_base);
        update_knowledge_base(x, y + 1, &perceived, knowledge_base);

        if predicate_glittery_and_safe_path(x, y + 1, knowledge_base) {
            return (x, y + 1);
        } else if predicate_glittery_and_safe_path(x, y - 1, knowledge_base) {
            return (x, y - 1);
        } else if predicate_glittery_and_safe_path(x + 1, y, knowledge_base) {
            return (x + 1, y);
        } else if predicate_glittery_and_safe_path(x - 1, y, knowledge_base) {
            return (x - 1, y);
        };

        if predicate_throw_arrow(x - 1, y, knowledge_base, num_of_arrows) {
            return (x - 1, y);
        } else if predicate_throw_arrow(x + 1, y, knowledge_base, num_of_arrows) {
            return (x + 1, y);
        } else if predicate_throw_arrow(x, y - 1, knowledge_base, num_of_arrows) {
            return (x, y - 1);
        } else if predicate_throw_arrow(x, y + 1, knowledge_base, num_of_arrows) {
            return (x, y + 1);
        }

        if predicate_safe_unvisited_path(x, y + 1, knowledge_base) {
            return (x, y + 1);
        } else if predicate_safe_unvisited_path(x, y - 1, knowledge_base) {
            return (x, y - 1);
        } else if predicate_safe_unvisited_path(x + 1, y, knowledge_base) {
            return (x + 1, y);
        } else if predicate_safe_unvisited_path(x - 1, y, knowledge_base) {
            return (x - 1, y);
        };

        print!("Found no unvisited safe node so backtracking...");
        if backtrack(x, y + 1, knowledge_base) {
            return (x, y + 1);
        } else if backtrack(x, y - 1, knowledge_base) {
            return (x, y - 1);
        } else if backtrack(x + 1, y, knowledge_base) {
            return (x + 1, y);
        } else if backtrack(x - 1, y, knowledge_base) {
            return (x - 1, y);
        };
    } else {
        print!("START MAKING LOOP PATH");

        let mut frontier_cells = find_frontier_cells(&knowledge_base);
        let mut rng = rand::thread_rng();
        frontier_cells.shuffle(&mut rng);

        if let Some(mut target) = find_least_dangerous_location(&frontier_cells, &knowledge_base) {

            if knowledge_base[target.0][target.1].visitedCount > 10 {
                frontier_cells.shuffle(&mut rng);
                match frontier_cells.last() {
                    Some(last_element) => {
                        print!("!!!GIVING RANDOM MOVEEE");
                        target = *last_element;
                    }
                    None => {
                        println!("The vector is empty.");
                    }
                }
            }

            if let Some(path) =
                find_shortest_path_bfs(x as usize, y as usize, target, &knowledge_base)
            {
                let _ = save_bfs_path_to_file(path, &String::from("bfs_path.txt"));

                return (1, 1); // Returning dummy value
            }
            if let Some((next_x, next_y)) = frontier_cells.get(0) {
                return (*next_x as i32, *next_y as i32);
            }
        }
    }

    print!("CONTROL REACHED HERE! BUGGGG-----");
    let probably_dangerous_paths = exclude_death_paths(x, y, knowledge_base);
    if probably_dangerous_paths.len() != 0 {
        return (
            probably_dangerous_paths[0].0 as i32,
            probably_dangerous_paths[0].1 as i32,
        );
    };
    // Simply Backtrack == BAD. TODO
    // if backtrack(x, y + 1, knowledge_base) {
    //     return (x, y + 1);
    // } else if backtrack(x, y - 1, knowledge_base) {
    //     return (x, y - 1);
    // } else if backtrack(x + 1, y, knowledge_base) {
    //     return (x + 1, y);
    // } else if backtrack(x - 1, y, knowledge_base) {
    //     return (x - 1, y);
    // };
    return (1, 1);
}
