use std::{fs, path::PathBuf};

use crate::utils::term::print_status;

pub mod puzzle_one {
    use crate::utils::term::{print_result, print_status, print_start};
    use super::{read_reports, check_asc, is_valid_diff};

    // https://adventofcode.com/2024/day/2
    pub fn solve() -> () {
        print_start("Day 2, Puzzle 1");
        print_status("Reading file");

        let reports = read_reports();
        print_status("Checking reports for safety");
        let mut total_safe = 0;
        for report in reports.iter() {
            if is_report_safe(report) {
                total_safe += 1;
            }
        };

        print_result(&total_safe.to_string());
    }

    // A report is safe if the levels are only ASC or DESC
    // AND the abs_diff is 1 - 3 between each level
    fn is_report_safe(report: &Vec<u32>) -> bool {
        // Get first two
        let first_level = report[0];
        let second_level = report[1];
        let is_asc = check_asc(first_level, second_level);

        let valid_diff = is_valid_diff(first_level, second_level);
        if !valid_diff {
            // Right off the bat, this report is not safe
            return false;
        };

        // Now, we have to check the remaining levels to
        // make sure that they respect the same rules
        let mut last_idx = 1;
        'level_check: loop {
            let this_level = report[last_idx];
            let next_level = report[last_idx + 1];

            let this_is_asc = check_asc(this_level, next_level);
            if this_is_asc != is_asc {
                return false;
            };

            if !is_valid_diff(this_level, next_level) {
                return false;
            };

            last_idx += 1;
            if last_idx == report.len() - 1 {
                break 'level_check;
            }
        }
        return true;
    }

}

pub mod puzzle_two {
    // https://adventofcode.com/2024/day/2#part2
    // The Problem Dampener is a module that tolerates a single bad level.
    // If we can get rid of a single level, the report can be considered safe.
    #[allow(dead_code)]
    pub fn solve() -> () {
        panic!("🔴 Day 2, Puzzle 2 not implemented");
    }
}

// Read the reports from the file as a 2D array
// of numbers
fn read_reports() -> Vec<Vec<u32>> {
    // Get the file path
    let app_dir = std::env::current_dir().expect("🔴 Could not get current directory");
    let report_file_path = app_dir.join(
        PathBuf::from("src/days/day_two_reports.txt")
    );

    // Read file content by line
    let content = fs::read_to_string(report_file_path).expect("🔴 Could not read file");
    let content_lines : Vec<&str> = content.split("\n").collect();

    // Parse the content. Each line is a space-separated list of numbers.
    // Each line represents a report, and each number represents a "level".
    print_status("Parsing report lines");
    let mut reports : Vec<Vec<u32>> = vec![];
    for line in content_lines.iter() {
        let this_line_level_strings : Vec<&str> = line.split(" ").collect();
        let this_line_level_nums = this_line_level_strings.iter().map(|s| -> u32 {
            let num = s.parse::<u32>().expect("🔴 Could not parse number");
            return num;
        });

        let mut this_report : Vec<u32> = vec![];
        this_line_level_nums.for_each(|num| -> () {
            this_report.push(num);
        });

        reports.push(this_report);
    };

    return reports;
}

fn check_asc(first: u32, second: u32) -> bool {
    return first < second;
}

fn is_valid_diff(first: u32, second: u32) -> bool {
    let diff = first.abs_diff(second);
    return diff >= 1 && diff <= 3;
}
