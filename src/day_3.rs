use crate::get_input;
use std::{
    collections::{HashMap, HashSet},
    io::Result,
};

const INPUT: &str = "inputs/day_3.txt";
const TEST: &str = "inputs/test.txt";

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

// Elapsed time: 13315 us
// Memory Used: 393.9746 kb
pub fn sum_part_numbers() -> Result<u32> {
    let schematic = get_input(INPUT)?;
    let part_regex = regex::Regex::new(r"[^\d.]").unwrap();
    let part_locs: HashSet<(isize, isize)> = schematic
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            part_regex
                .find_iter(line)
                .map(|m| (i as isize, m.start() as isize))
                .collect::<HashSet<(isize, isize)>>()
        })
        .collect();

    let number_regex = regex::Regex::new(r"\d+").unwrap();
    Ok(schematic
        .lines()
        .enumerate()
        .map(|(i, line)| {
            number_regex
                .find_iter(line)
                .filter_map(|m| {
                    let num = m.as_str().parse::<u32>().unwrap();
                    if (m.start()..m.end()).any(|j| {
                        NEIGHBORS
                            .iter()
                            .any(|(x, y)| part_locs.contains(&((i as isize) + x, j as isize + y)))
                    }) {
                        Some(num)
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum())
}

// Elapsed time: 9672 us
// Memory Used: 339.02344 kb
pub fn sum_gear_ratios() -> Result<u32> {
    let schematic = get_input(INPUT)?;
    let number_regex = regex::Regex::new(r"\d+").unwrap();
    let num_loc_to_num: HashMap<(isize, isize), u32> = schematic
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            number_regex
                .find_iter(line)
                .flat_map(|m| {
                    (m.start()..m.end())
                        .map(|j| ((i as isize, j as isize), m.as_str().parse::<u32>().unwrap()))
                        .collect::<Vec<((isize, isize), u32)>>()
                })
                .collect::<Vec<((isize, isize), u32)>>()
        })
        .collect();

    let gear_regex = regex::Regex::new(r"\*").unwrap();
    Ok(schematic
        .lines()
        .enumerate()
        .map(|(i, line)| {
            gear_regex
                .find_iter(line)
                .filter_map(|m| {
                    let neighbors = NEIGHBORS
                        .iter()
                        .filter_map(|(x, y)| {
                            match num_loc_to_num.get(&((i as isize) + x, m.start() as isize + y)) {
                                Some(v) => Some(*v),
                                None => None,
                            }
                        })
                        .collect::<HashSet<u32>>();
                    if neighbors.len() == 2 {
                        Some(neighbors.iter().product::<u32>())
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum())
}
