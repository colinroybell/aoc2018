use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day1a() {
        assert_eq!(3,day1a("input/01a_test1.txt"));
        assert_eq!(0,day1a("input/01a_test2.txt"));
        assert_eq!(-6,day1a("input/01a_test3.txt"));    
    }

    #[test]
    fn test_day1b() {
        assert_eq!(0,day1b("input/01b_test1.txt"));
        assert_eq!(10,day1b("input/01b_test2.txt"));
        assert_eq!(5,day1b("input/01b_test3.txt")); 
        assert_eq!(14,day1b("input/01b_test4.txt"));     
    }
}

// Credit to /r/Dutch_GhOst
fn day1a(filename: &str) -> isize {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);

    // parse -> Result<isize,ParseError>
    // ok takes Result to Option<> or None
    // filter_map then drops the None and returns the value
    // isize is a signed integer of the same size as an
    //   address on the machine. usize needed for addressing.
    // option is either some(V) or None
    let mut text :String = String::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        text.push_str(&line);
        text.push_str("\n")
    }
 
    let sum = text
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .sum::<isize>();

    sum
}        

fn day1b(filename: &str) -> isize {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);

    let mut text :String = String::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        text.push_str(&line);
        text.push_str("\n")
    }
   
    // This works if we do it in text but not if we start with the file
    let mut set = HashSet::new();
    let mut sum: isize = 0;
    for n in text
        .lines()
        .filter_map(|s| s.parse::<isize>().ok()).cycle()
        {  
        if set.contains(&sum) {
            return sum;
        }    
        set.insert(sum);
        sum += n;
    }        
    -999
}




pub fn run() {  
    println!("Part 1 answer: {:?}", day1a("input/01a.txt"));
    println!("Part 2 answer: {:?}", day1b("input/01a.txt")); 
}
