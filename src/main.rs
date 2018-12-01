use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::collections::HashSet;

fn day1a(filename: &str) -> i32 {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);

    let mut sum:i32 = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let num:i32 = line.parse().unwrap();
        sum += num
    }
    
    sum
}        

fn day1b(filename: &str) -> i32 {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);

    let mut nums:Vec<i32> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let num:i32 = line.parse().unwrap();
        nums.push(num);
    }

    let mut sum:i32 = 0;
    let mut seen = HashSet::new();
    loop {
        for num in &mut nums {
            let num:i32 = *num;
          
            if seen.contains(&sum) {
                return sum;
            } else {
                seen.insert(sum);
            }
            sum += num;
        }
    }
}

fn main() {
    assert_eq!(3,day1a("input/01a_test1.txt"));
    assert_eq!(0,day1a("input/01a_test2.txt"));
    assert_eq!(-6,day1a("input/01a_test3.txt"));

    println!("Part 1 answer: {:?}", day1a("input/01a.txt"));

    assert_eq!(0,day1b("input/01b_test1.txt"));
    assert_eq!(10,day1b("input/01b_test2.txt"));
    assert_eq!(5,day1b("input/01b_test3.txt")); 
    assert_eq!(14,day1b("input/01b_test4.txt")); 
 
    println!("Part 2 answer: {:?}", day1b("input/01a.txt")); 
}
