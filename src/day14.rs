use crate::utils::day_setup::Utils;
use std::collections::HashMap;
use std::slice::Iter;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/14).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 14, Some(3118));
    Utils::run_part_single(part2, 2, 14, Some(4332887448171));
}

fn part1(mut polymer_formula: PolymerFormula) -> u64 {
    const COUNT: u8 = 10;

    simulate::<{ COUNT }>(&mut polymer_formula);

    let (min, max) = polymer_formula.polymer_template.min_max_occurrence();

    max - min
}

fn part2(mut polymer_formula: PolymerFormula) -> u64 {
    const COUNT: u8 = 40;

    simulate::<{ COUNT }>(&mut polymer_formula);

    let (min, max) = polymer_formula.polymer_template.min_max_occurrence();

    max - min
}

fn simulate<const COUNT: u8>(polymer_formula: &mut PolymerFormula) {
    let mut new_points = Vec::new();
    let mut points_to_remove = Vec::new();
    for _ in 0..COUNT {
        for ((a, b), count) in polymer_formula.polymer_template.template.iter() {
            if let Some(new) =
                PolymerFormula::get_replacement(&polymer_formula.insertion_rules, (*a, *b))
            {
                polymer_formula.polymer_template.element_count[(new as u8 - b'A') as usize] +=
                    count;
                points_to_remove.push((*a, *b));
                new_points.push(([(*a, new), (new, *b)], *count));
            }
        }

        polymer_formula
            .polymer_template
            .update_points(&new_points, &points_to_remove);

        new_points.clear();
        points_to_remove.clear();
    }
}

struct PolymerFormula {
    polymer_template: PolymerTemplate,
    insertion_rules: HashMap<(char, char), char>,
}

impl PolymerFormula {
    pub fn get_replacement(
        insertion_rules: &HashMap<(char, char), char>,
        (first, second): (char, char),
    ) -> Option<char> {
        if let Some(replacement) = insertion_rules.get(&(first, second)) {
            return Some(*replacement);
        }
        None
    }

    fn extract_rules(input: &mut Iter<String>) -> HashMap<(char, char), char> {
        input
            .map(|line| {
                let mut line = line.chars();
                let first = line.next().unwrap();
                let second = line.next().unwrap();
                let result = line.last().unwrap();
                ((first, second), result)
            })
            .collect::<HashMap<_, _>>()
    }
}

struct PolymerTemplate {
    template: HashMap<(char, char), u64>,
    element_count: [u64; 26],
}

impl PolymerTemplate {
    pub fn update_points(
        &mut self,
        new_points: &[([(char, char); 2], u64)],
        points_to_remove: &[(char, char)],
    ) {
        // First, remove the points that need to be removed
        for points in points_to_remove {
            self.template.remove(points);
        }

        // Then, add the new points
        for (new_points, count) in new_points {
            for pair in new_points {
                *self.template.entry(*pair).or_insert(0) += *count;
            }
        }
    }

    fn min_max_occurrence(&self) -> (u64, u64) {
        let min = self
            .element_count
            .iter()
            .filter(|&&x| x != 0)
            .min()
            .unwrap();

        let max = self.element_count.iter().max().unwrap();

        (*min, *max)
    }
}

impl From<Vec<String>> for PolymerFormula {
    fn from(input: Vec<String>) -> Self {
        let mut count = [0; 26];

        let mut iter = input.iter();
        let binding = iter.next().unwrap().chars().collect::<Vec<char>>();

        binding
            .iter()
            .for_each(|&c| count[(c as u8 - b'A') as usize] += 1);

        let mut polymer_template = HashMap::new();

        binding.windows(2).for_each(|window: &[char]| match window {
            [x, y] => {
                *polymer_template.entry((*x, *y)).or_insert(0) += 1;
            }
            _ => unreachable!("windows(2) should always yield a slice of exactly 2 elements"),
        });

        assert!(iter.next().unwrap().is_empty());

        PolymerFormula {
            polymer_template: PolymerTemplate {
                template: polymer_template,
                element_count: count,
            },
            insertion_rules: PolymerFormula::extract_rules(&mut iter),
        }
    }
}
