use crate::utils::day_setup::Utils;
use crate::utils::graph::{Graph, Neighbours, NodePtr, Relationship};
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::sync::mpsc::{Receiver, Sender};

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
    let mut path_builder = PathsBuilder::new();

    std::thread::scope(|scope| {
        cave_map
            .get_nodes()
            .iter()
            .filter(|node| matches!(node, Cave::Small(_)))
            .map(|cave| (cave, path_builder.new_path()))
            .for_each(|(repeat_cave, path)| {
                scope.spawn(|| {
                    let mut path = path;
                    distinct_path_with_options(
                        &cave_map,
                        &cave_map.start,
                        &mut path,
                        (repeat_cave, 2),
                    );
                });
            });
    });

    path_builder.build()
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
    path: &mut Path,
    (repeat_node, mut count): (&Cave, i32),
) {
    path.add_to_path(cave_map.get_node_data(curr_index).clone());

    if *curr_index == cave_map.end {
        path.send();
        return;
    }

    for (curr_node_index, _) in cave_map.neighbours(curr_index) {
        // Cannot move though the start state & cannot pass through small caves more than once
        let current_cave = cave_map.get_node_data(curr_node_index);

        if repeat_node == current_cave && count != 0 {
            count -= 1;
            // Continue on as you haven't visited the node twice
        } else if path.contains(curr_node_index) || *curr_node_index == cave_map.start {
            continue;
        }

        if matches!(cave_map.map.get(curr_node_index), Cave::Small(_)) {
            path.add_to_visited(curr_node_index.clone());
        }

        distinct_path_with_options(cave_map, curr_node_index, path, (repeat_node, count));
        path.pop_path();

        if repeat_node == current_cave {
            count += 1;
        }
    }

    if matches!(cave_map.map.get(curr_index), Cave::Small(_)) {
        path.remove_from_visited(curr_index);
    }
}

/// A builder for managing and storing paths in the cave system.
///
/// This struct is responsible for creating new paths, storing the final paths,
/// and sending the completed paths through a channel.
struct PathsBuilder {
    /// A set of final paths represented as strings.
    final_paths: HashSet<String>,
    /// An optional sender channel to send the completed paths.
    tx: Option<Sender<Vec<String>>>,
    /// A receiver channel to receive the completed paths.
    rx: Receiver<Vec<String>>,
}

/// Represents a path in the cave system.
///
/// This struct is used to build and store a path through the cave system,
/// sending the completed path through a channel when finished.
struct Path {
    /// The sender channel to send the completed path.
    tx: Sender<Vec<String>>,
    /// The current path being built.
    path: Vec<String>,
    /// A set of visited nodes in the current path.
    visited: HashSet<NodePtr>,
}

impl Path {
    /// Adds a cave to the current path.
    ///
    /// # Arguments
    /// * `cave` - The cave to add to the path.
    fn add_to_path(&mut self, cave: Cave) {
        self.path.push(format!("{:?}", cave));
    }

    /// Removes a node from the set of visited nodes.
    ///
    /// # Arguments
    /// * `node_ptr` - The node to remove from the visited set.
    fn remove_from_visited(&mut self, node_ptr: &NodePtr) {
        self.visited.remove(node_ptr);
    }

    /// Checks if a node is in the set of visited nodes.
    ///
    /// # Arguments
    /// * `node_ptr` - The node to check.
    ///
    /// # Returns
    /// `true` if the node is in the visited set, `false` otherwise.
    fn contains(&mut self, node_ptr: &NodePtr) -> bool {
        self.visited.contains(node_ptr)
    }

    /// Removes the last cave from the current path.
    ///
    /// # Panics
    /// Panics if the path is empty.
    fn pop_path(&mut self) {
        assert!(self.path.pop().is_some());
    }

    /// Adds a node to the set of visited nodes.
    ///
    /// # Arguments
    /// * `node_ptr` - The node to add to the visited set.
    fn add_to_visited(&mut self, node_ptr: NodePtr) {
        self.visited.insert(node_ptr);
    }

    /// Sends the completed path through the channel.
    ///
    /// # Panics
    /// Panics if the channel fails to send the path.
    fn send(&self) {
        self.tx
            .send(self.path.clone())
            .expect("Failed to send path to build");
    }
}

impl PathsBuilder {
    /// Creates a new `PathsBuilder`.
    ///
    /// # Returns
    /// A new instance of `PathsBuilder`.
    fn new() -> PathsBuilder {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            final_paths: HashSet::new(),
            rx,
            tx: Some(tx),
        }
    }

    /// Creates a new `Path`.
    ///
    /// # Returns
    /// A new instance of `Path`.
    ///
    /// # Panics
    /// Panics if the channel sender no longer exists.
    fn new_path(&self) -> Path {
        Path {
            tx: self
                .tx
                .clone()
                .expect("Cannot create path as the Channel Sender no longer exists")
                .clone(),
            path: vec![],
            visited: HashSet::new(),
        }
    }

    /// Returns the number of final paths.
    ///
    /// # Returns
    /// The number of final paths.
    fn count(&self) -> usize {
        self.final_paths.len()
    }

    /// Builds the final paths by collecting them from the receiver channel.
    ///
    /// # Returns
    /// The number of final paths.
    fn build(&mut self) -> usize {
        drop(self.tx.take());
        for path in self.rx.iter() {
            self.final_paths
                .insert(path.iter().fold(String::new(), |acc, x| acc + x));
        }

        self.count()
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
