use std::fs::File;
use std::io::{BufRead,BufReader};
//use regex::Regex;
use std::collections::HashSet;

// Much better implementation - read into a Vec, compare 
// with last already there and if it's the same, delete
// both

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day5a() {
        assert_eq!(0,day5a("input/05a_test1.txt"));
        assert_eq!(0,day5a("input/05a_test2.txt"));
        assert_eq!(6,day5a("input/05a_test3.txt"));
        assert_eq!(10,day5a("input/05a_test4.txt"));
    }

    #[test]
    fn test_day5b() {
        assert_eq!(0,day5b("input/05a_test1.txt"));
    }
}   

fn iter(input: Vec<char>) -> (Vec<char>,bool) {
    let mut output: Vec<char> = Vec::new();
    let mut change = false;
    let mut current: Option<char> = None;
    for c in input {
        if current == None {
            current = Some(c);
        } else {
            if current.unwrap().is_uppercase() != c.is_uppercase() &&
            current.unwrap().to_uppercase().to_string() == c.to_uppercase().to_string() {
                // match. Discard both
                change = true;
                current = None;
            } else {
                output.push(current.unwrap());
                current = Some(c);
            }
        } 
    }
    if current != None {
        output.push(current.unwrap());
    }
    return (output,change);
}

fn calc(input: String, exclusion: char) -> u32 {

    let mut vec :Vec<char> = Vec::new();

    let exclusion_upper = exclusion.to_uppercase().collect::<Vec<_>>()[0];

    for c in input.chars() {
        if c != exclusion && c != exclusion_upper {
          vec.push(c);
        }
    }
    let mut change = true;
    while change {
        let res = iter(vec);
        vec = res.0;
        change = res.1;

    }
    return vec.len() as u32;
}

fn day5a(filename: &str) -> u32 {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
 
    let mut text :String = String::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        text.push_str(&line);
    }
  
    calc(text,'-')


}

    
fn day5b(filename: &str) -> u32 {    
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
 
    let mut text :String = String::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        text.push_str(&line);
    }
  
    let mut polymers: HashSet<char> = HashSet::new();

    for c in text.chars() {
        polymers.insert(c.to_lowercase().collect::<Vec<_>>()[0]);
    }

    let mut minimum: u32 = 1000000;

    for c in polymers {
        let passedText = &text;
        let res = calc(passedText.to_string(),c);
        println!("{} {}",c,res);
        if res < minimum {
            minimum = res
        }
    }
    
    minimum
}

pub fn run() {  
    println!("Part 1 answer: {:?}", day5a("input/05.txt"));
    println!("Part 2 answer: {:?}", day5b("input/05.txt")); 
}
