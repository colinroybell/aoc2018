use std::fs::File;
use std::io::{BufRead,BufReader};
//use regex::Regex;
//use std::collections::{HashSet,HashMap};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day7a() {
        assert_eq!(0,day6a("input/07a_test1.txt"));
    }

    #[test]
    fn test_day7b() {
        assert_eq!(0,day7b("input/07a_test1.txt"));
    }
}   

fn day7a(_filename: &str) -> usize {
   

}

    
fn day7b(_filename: &str) -> u32 {    
   0
}

pub fn run() {  
    println!("Part 1 answer: {:?}", day7a("input/07.txt"));
    println!("Part 2 answer: {:?}", day7b("input/07.txt")); 
}
