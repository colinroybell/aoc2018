use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day3a() {
        assert_eq!(4,day3a("input/03a_test1.txt"));
    }

    #[test]
    fn test_day3b() {
        assert_eq!(3,day3b("input/03b_test1.txt"))
    }
}


fn day3a(filename: &str) -> u32 {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
 
    const SIZE:usize = 1100;

    let mut claims:[[u32;SIZE];SIZE] = [[0;SIZE];SIZE];
    let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
 

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        println!("{}",line);
        assert!(re.is_match(&line));
        let cap = re.captures(&line).unwrap();
        let _claim:u32 = cap[1].parse().unwrap();
        let x_start:u32 = cap[2].parse().unwrap();
        let y_start:u32 = cap[3].parse().unwrap();
        let x_size:u32 = cap[4].parse().unwrap();
        let y_size:u32 = cap[5].parse().unwrap();

        println!("{} {} {} {}",x_start,y_start,x_size,y_size);
        assert!(((x_start+x_size) as usize) < SIZE);
        assert!(((y_start+y_size) as usize) < SIZE);
        for x in x_start..x_start+x_size {
            for y in y_start..y_start+y_size {
                claims[x as usize][y as usize] += 1;
            }
        }
    }

    let mut squares = 0;

    for x in 0..SIZE {
        for y in 0..SIZE {
            if claims[x][y]>1 {
                squares += 1;
            }
        }
    }
    squares
}

    
fn day3b(filename: &str) -> u32 { 
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
 
    const SIZE:usize = 1100;

    let mut claims:[[u32;SIZE];SIZE] = [[0;SIZE];SIZE];
    let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
 
    let mut good = HashSet::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        println!("{}",line);
        assert!(re.is_match(&line));
        let cap = re.captures(&line).unwrap();
        let claim:u32 = cap[1].parse().unwrap();
        let x_start:u32 = cap[2].parse().unwrap();
        let y_start:u32 = cap[3].parse().unwrap();
        let x_size:u32 = cap[4].parse().unwrap();
        let y_size:u32 = cap[5].parse().unwrap();

        good.insert(claim);

        println!("{} {} {} {}",x_start,y_start,x_size,y_size);
        assert!(((x_start+x_size) as usize) < SIZE);
        assert!(((y_start+y_size) as usize) < SIZE);
        for x in x_start..x_start+x_size {
            for y in y_start..y_start+y_size {
                let val = claims[x as usize][y as usize];
                if val > 0 {
                    good.remove(&val);
                    good.remove(&claim);
                }
                claims[x as usize][y as usize] = claim;
            }
        }
    } 

    let mut val = 0;

    assert!(good.len()==1);
 
    for x in good.iter() {
        val = *x;
    }

    val
}

pub fn run() {  
    println!("Part 1 answer: {:?}", day3a("input/03.txt"));
    println!("Part 2 answer: {:?}", day3b("input/03.txt")); 
}
