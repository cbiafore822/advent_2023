use crate::get_input;
use std::io::Result;

const INPUT: &str = "inputs/day_1.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 1059 us
// Memory Used: 21.475586 kb
pub fn get_calibration_value() -> Result<u32> {
    let calibrations = get_input(INPUT)?;
    Ok(calibrations
        .lines()
        .map(|calibration| {
            let chars = calibration.chars();
            let first = chars.clone().find(|c| c.is_ascii_digit()).unwrap();
            let last = chars.into_iter().rfind(|c| c.is_ascii_digit()).unwrap();

            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum())
}

// Elapsed time: 2087 us
// Memory Used: 52.614258 kb
pub fn get_calibration_value_with_words() -> Result<u32> {
    let mut calibrations = get_input(INPUT)?;
    calibrations = calibrations.replace("one", "o1e");
    calibrations = calibrations.replace("two", "t2o");
    calibrations = calibrations.replace("three", "t3e");
    calibrations = calibrations.replace("four", "f4r");
    calibrations = calibrations.replace("five", "f5e");
    calibrations = calibrations.replace("six", "s6x");
    calibrations = calibrations.replace("seven", "s7n");
    calibrations = calibrations.replace("eight", "e8t");
    calibrations = calibrations.replace("nine", "n9e");
    Ok(calibrations
        .lines()
        .map(|calibration| {
            let chars = calibration.chars();
            let first = chars.clone().find(|c| c.is_ascii_digit()).unwrap();
            let last = chars.into_iter().rfind(|c| c.is_ascii_digit()).unwrap();

            format!("{}{}", first, last,).parse::<u32>().unwrap()
        })
        .sum())
}
