use crate::get_input;
use regex::Regex;
use std::{collections::HashMap, io::Result};

const INPUT: &str = "inputs/day_2.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 8333 us
// Memory Used: 347.98535 kb
pub fn get_possible_games() -> Result<u32> {
    let games = get_input(INPUT).unwrap();
    let game_id_regex = Regex::new(r"\d+").unwrap();
    let game_regex = Regex::new(r"\d+ (blue|red|green)").unwrap();
    let color_limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    Ok(games
        .lines()
        .filter_map(|game| {
            let game_id = game_id_regex
                .find(game)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            if game_regex.find_iter(game).all(|m| {
                let (num, color) = m.as_str().split_once(' ').unwrap();
                num.parse::<u32>().unwrap() <= color_limits[color]
            }) {
                Some(game_id)
            } else {
                None
            }
        })
        .sum())
}

// Elapsed time: 7554 us
// Memory Used: 334.91504 kb
pub fn get_power_of_games() -> Result<u32> {
    let games = get_input(INPUT).unwrap();
    let game_regex = Regex::new(r"\d+ (blue|red|green)").unwrap();
    Ok(games
        .lines()
        .map(|game| {
            let mut highest = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            game_regex.find_iter(game).for_each(|m| {
                let (num, color) = m.as_str().split_once(' ').unwrap();
                let num = num.parse::<u32>().unwrap();
                highest.entry(color).and_modify(|e| *e = num.max(*e));
            });
            highest.values().product::<u32>()
        })
        .sum())
}
