use crate::utils::day_setup::Utils;
use crate::utils::graph::{Graph, Neighbours, NodePtr, Relationship};
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/12).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 12, Some(4691));
    Utils::run_part_single(part2, 2, 12, Some(140718));
}

fn part1(cave_map: CaveMap) -> u64 {
    let mut small_caves_stack: Vec<NodePtr> = Vec::with_capacity(cave_map.map.len());
    distinct_path_once(&cave_map, &cave_map.start, &mut small_caves_stack)
}

fn part2(cave_map: CaveMap) -> usize {
    let mut small_caves_stack: Vec<NodePtr> = Vec::with_capacity(cave_map.map.len());
    let mut path_storage = PathsBuilder::new();

    cave_map
        .get_nodes()
        .iter()
        .filter(|node| {
            !matches!(node, Cave::Start)
                & !matches!(node, Cave::End)
                & !matches!(node, Cave::Big(_))
        })
        .for_each(|repeat| {
            distinct_path_with_options(
                &cave_map,
                &cave_map.start,
                &mut small_caves_stack,
                &mut path_storage,
                (repeat, 2),
            );

            small_caves_stack.clear();
            path_storage.reset_temp();
        });

    path_storage.count()
}

fn distinct_path_once(
    cave_map: &CaveMap,
    curr_index: &NodePtr,
    small_caves_stack: &mut Vec<NodePtr>,
) -> u64 {
    if *curr_index == cave_map.end {
        return 1;
    }

    let mut result = 0;
    for (curr_node_index, _) in cave_map.neighbours(curr_index) {
        // Cannot move though the start state & cannot pass through small caves more than once
        if small_caves_stack.contains(curr_node_index) || *curr_node_index == cave_map.start {
            continue;
        }

        if matches!(cave_map.map.get(curr_node_index), Cave::Small(_)) {
            small_caves_stack.push(curr_node_index.clone());
        }

        result += distinct_path_once(cave_map, curr_node_index, small_caves_stack);
    }

    if matches!(cave_map.map.get(curr_index), Cave::Small(_)) {
        small_caves_stack.pop();
    }

    result
}

fn distinct_path_with_options(
    cave_map: &CaveMap,
    curr_index: &NodePtr,
    small_caves_stack: &mut Vec<NodePtr>,
    path_storage: &mut PathsBuilder,
    (repeat_node, mut count): (&Cave, i32),
) {
    path_storage.add_to_path(cave_map.get_node_data(curr_index).clone());

    if *curr_index == cave_map.end {
        path_storage.build_path();
        return;
    }

    for (curr_node_index, _) in cave_map.neighbours(curr_index) {
        // Cannot move though the start state & cannot pass through small caves more than once
        let current_cave = cave_map.get_node_data(curr_node_index);

        if repeat_node == current_cave && count != 0 {
            count -= 1;
            // Continue on as you haven't visited the node twice
        } else if small_caves_stack.contains(curr_node_index) || *curr_node_index == cave_map.start
        {
            continue;
        }

        if matches!(cave_map.map.get(curr_node_index), Cave::Small(_)) {
            small_caves_stack.push(curr_node_index.clone());
        }

        distinct_path_with_options(
            cave_map,
            curr_node_index,
            small_caves_stack,
            path_storage,
            (repeat_node, count),
        );
        path_storage.pop_path();

        if repeat_node == current_cave {
            count += 1;
        }
    }

    if matches!(cave_map.map.get(curr_index), Cave::Small(_)) {
        small_caves_stack.pop();
    }
}

struct PathsBuilder {
    final_paths: HashSet<String>,
    curr_path_building: Vec<String>,
}

impl PathsBuilder {
    fn new() -> PathsBuilder {
        Self {
            curr_path_building: vec![],
            final_paths: HashSet::new(),
        }
    }

    fn reset_temp(&mut self) {
        self.curr_path_building.clear();
    }

    fn count(&self) -> usize {
        self.final_paths.len()
    }

    fn add_to_path(&mut self, cave: Cave) {
        self.curr_path_building.push(format!("{:?}", cave));
    }

    fn pop_path(&mut self) {
        assert!(self.curr_path_building.pop().is_some());
    }

    fn build_path(&mut self) {
        let new_path = self
            .curr_path_building
            .iter()
            .fold(String::new(), |acc, x| acc + x);
        self.final_paths.insert(new_path);
    }
}

impl Debug for PathsBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Paths to End {{")?;
        for path in self.final_paths.iter() {
            writeln!(f, "\t{:?}", path)?;
        }
        write!(f, "}}")
    }
}

#[derive(Debug)]
struct CaveMap {
    map: Graph<Cave, ()>,
    start: NodePtr,
    end: NodePtr,
}

impl CaveMap {
    fn neighbours(&self, curr_index: &NodePtr) -> Neighbours<'_, Cave, ()> {
        self.map.neighbours_iter(curr_index)
    }

    fn get_nodes(&self) -> Vec<&Cave> {
        self.map.nodes()
    }

    fn get_node_data(&self, node_ptr: &NodePtr) -> &Cave {
        self.map.get(node_ptr)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Cave {
    End,
    Start,
    Big(String),
    Small(String),
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cave::End => write!(f, "End"),
            Cave::Start => {
                write!(f, "Start")
            }
            Cave::Big(big) => {
                write!(f, "Big({})", big)
            }
            Cave::Small(small) => {
                write!(f, "Small({})", small)
            }
        }
    }
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
            .map(|points| {
                let (from, to) = points.split_once('-').unwrap();
                (
                    Cave::from(from.to_string()),
                    Cave::from(to.to_string()),
                    Relationship::BiDirectional {
                        a_to_b: (),
                        b_to_a: (),
                    },
                )
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
