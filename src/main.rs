// import lib
mod days;
mod utils;

// shortening the path(s)
use days::{
    day_one,
    day_two
};

fn main() {
    // Solve puzzles for day one
    day_one::puzzle_one::solve();
    day_one::puzzle_two::solve();

    // Solve puzzles for day two
    day_two::puzzle_one::solve();
}
