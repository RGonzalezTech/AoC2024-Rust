use std::{fs, path::PathBuf};

pub mod puzzle_one {
    use crate::utils::term::{print_result, print_status, print_start};
    use super::read_lists;

    // https://adventofcode.com/2024/day/1
    // Calculate the total absolute difference between each pair of 
    // numbers in two lists
    pub fn solve() -> () {
        print_start("Day 1: Puzzle 1");
        print_status("Reading lists from file");
        let ( mut left_list, mut right_list ) = read_lists();

        print_status("Sorting lists");
        // Sort the lists
        left_list.sort();
        right_list.sort();

        print_status("Calculating distances");
        let mut distances: Vec<u32> = Vec::new();
        let mut idx = 0;

        while idx < left_list.len() {
            let this_val = left_list[idx];
            let pair_val = right_list[idx];
            let distance = this_val.abs_diff(pair_val);

            distances.push(distance);
            idx += 1;
        }

        print_status("Calculating sum of distances");
        idx = 0; // reset idx
        let mut sum_of_diffs: u32 = 0;
        while idx < distances.len() {
            sum_of_diffs += distances[idx];
            idx += 1;
        };

        print_result(&sum_of_diffs.to_string());
    }
}

pub mod puzzle_two {
    use std::collections::HashMap;

    use crate::utils::term::{print_result, print_status, print_start};
    use super::read_lists;

    // https://adventofcode.com/2024/day/1#part2
    // Count occurences in right list of each number in the left list
    pub fn solve() -> () {
        print_start("Day 1: Puzzle 2");

        print_status("Loading lists");
        let ( left_list, right_list ) = read_lists();

        print_status("Pre-Counting Right List Occurences");
        let mut right_list_counts : HashMap<u32, u32> = HashMap::new();
        for location_id in right_list.iter() {
            let has_key = right_list_counts.contains_key(&location_id);
            if !has_key {
                // first occurence, count = 1
                right_list_counts.insert(*location_id, 1);
            } else {
                // Should have a value, cause we just checked
                let current_count = right_list_counts.get(location_id).expect("🔴 Could not get current-count");
                let new_count = *current_count + 1; // increment count
                right_list_counts.insert(*location_id, new_count); // update count
            };
        };

        print_status("Summing similarity scores");
        let mut total_similarity : u32 = 0;
        for location_id in left_list.iter() {
            let had_count = right_list_counts.contains_key(location_id);
            if !had_count {
                // skip
                continue;
            } else {
                // Should have a value, cause we just checked
                let id_count = right_list_counts.get(location_id).expect("🔴 Could not get count on request");
                let similarity_score = location_id * id_count;
                total_similarity += similarity_score; // sum up
            };
        };

        print_result(&total_similarity.to_string());
    }
}

// Read the 2 lists from the 1 text file
pub fn read_lists() -> (Vec<u32>, Vec<u32>) {
    let app_dir = std::env::current_dir().expect("🔴 Could not get current directory");
    let list_file_path = app_dir.join(
        PathBuf::from("src/utils/day_one_notes.txt")
    );

    let content = fs::read_to_string(list_file_path).expect("🔴 Could not read file");
    let content_lines : Vec<&str> = content.split("\n").collect();

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();
    let mut line_idx = 0;
    while line_idx < content_lines.len() {
        let line = content_lines[line_idx];

        // Split line into left & right
        let line_parts: Vec<&str> = line.split(" ").collect();
        let left_val = line_parts[0].parse().expect("🔴 Could not parse number (Left)");
        let right_val = line_parts[1].parse().expect("🔴 Could not parse number (Right)");

        left_list.push(left_val);
        right_list.push(right_val);
        line_idx += 1;
    };

    (left_list, right_list)
}