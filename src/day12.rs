use crate::utils::day_setup::Utils;
use crate::utils::graph::{Graph, Neighbours, NodeIndex};

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/12).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 12, 4691);
    Utils::run_part(part2, 2, 0, 0);
}

fn part1(cave_map: CaveMap) -> u64 {
    let mut small_caves_stack: Vec<NodeIndex> = Vec::with_capacity(cave_map.map.num_of_nodes());
    distinct_path_once(&cave_map, cave_map.start, &mut small_caves_stack)
}

fn part2(input: Vec<String>) -> u64 {
    // println!("Part 2 {:#?}", input);
    0
}

fn distinct_path_once(cave_map: &CaveMap, curr_index: NodeIndex, small_caves_stack: &mut Vec<NodeIndex>) -> u64 {
    if curr_index == cave_map.end {
        return 1;
    }

    let mut result = 0;
    for curr_node_index in cave_map.neighbours(curr_index) {
        // Cannot move though the start state & cannot pass through small caves more than once
        if small_caves_stack.contains(&curr_node_index) || curr_node_index == cave_map.start {
            continue;
        }

        if let Cave::Small(_) = cave_map.map.get_node_data(curr_node_index) {
            small_caves_stack.push(curr_node_index);
        }

        result += distinct_path_once(cave_map, curr_node_index, small_caves_stack);
    }
    
    if let Cave::Small(_) = cave_map.map.get_node_data(curr_index) {
        small_caves_stack.pop();
    }

    result
}

#[derive(Debug)]
struct CaveMap {
    map: Graph<Cave, ()>,
    start: NodeIndex,
    end: NodeIndex,
}

impl CaveMap {
    fn neighbours(&self, curr_index: NodeIndex) -> Neighbours<'_, Cave, ()> {
        self.map.neighbours_iter(curr_index)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Cave {
    End,
    Start,
    Big(String),
    Small(String),
}

impl From<String> for Cave {
    fn from(value: String) -> Self {
        match value.as_str() {
            "start" => Cave::Start,
            "end" => Cave::End,
            _ => {
                if value.chars().next().unwrap().is_ascii_lowercase() {
                    Cave::Small(value)
                } else {
                    Cave::Big(value)
                }
            }
        }
    }
}

impl From<Vec<String>> for CaveMap {
    fn from(value: Vec<String>) -> Self {
        let points = value
            .into_iter()
            .flat_map(|points| {
                let (from, to) = points.split_once('-').unwrap();
                [
                    (Cave::from(from.to_string()), Cave::from(to.to_string())),
                    (Cave::from(to.to_string()), Cave::from(from.to_string()))
                ]
            })
            .collect::<Vec<_>>();
        let graph = Graph::from(points);
        CaveMap {
            start: graph.find_node_index(|data| data == &Cave::Start).unwrap(),
            end: graph.find_node_index(|data| data == &Cave::End).unwrap(),
            map: graph,
        }
    }
}
