const PUZZLE: &str = include_str!("input.txt");
use std::cell::Cell;
use std::collections::HashSet;

// for part 1
fn main() {
    let sum = PUZZLE.lines().filter_map(|s| s.parse::<isize>().ok()).sum::<isize>();
    println!("{}", sum);
}

// for part 2
fn main() {
    let mut set = HashSet::new();

    let frequency = Cell::new(0);

    PUZZLE
        .lines()
        .flat_map(|s| s.parse::<isize>().ok())
        .cycle()
        .take_while(|_| set.insert(frequency.get()))
        .for_each(|n| {
            frequency.update(|old| old+n);
        });

    println!("{:?}", frequency);
}    

// Questions:
// filter_map vs flat_map.
// parse::<isize>, sum::<isize>
// take_while?