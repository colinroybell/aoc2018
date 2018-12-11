use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day10a() {
        assert_eq!(1, day10a("input/10_test1.txt",0,4));
//        assert_eq!(0,day6a("input/07a_test1.txt"));
    }

    #[test]
    fn test_day10b() {
//        assert_eq!(0,day7b("input/07a_test1.txt"));
    }
}   

struct Point {
    start_x: isize,
    start_y: isize,
    velocity_x: isize,
    velocity_y: isize
}

impl Point {
    pub fn new(start_x: isize, start_y: isize, velocity_x: isize, velocity_y: isize) -> Point {
        Point {
            start_x: start_x,
            start_y: start_y,
            velocity_x: velocity_x,
            velocity_y: velocity_y
        }
    }
}

fn day10a(filename: &str, t1: usize, t2:usize) -> usize {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
     let re1 = Regex::new(r"position=<\s*([-\d]+),\s*([-\d]+)>\s+velocity=<\s*([-\d]+),\s*([-\d]+)>").unwrap();
    let mut points :Vec<Point> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        assert!(re1.is_match(&line));
        let cap = re1.captures(&line).unwrap();
        let start_x: isize = cap[1].parse().unwrap();
        let start_y: isize = cap[2].parse().unwrap();
        let velocity_x: isize = cap[3].parse().unwrap();
        let velocity_y: isize = cap[4].parse().unwrap();   
        let point = Point::new(start_x, start_y, velocity_x, velocity_y);
        points.push(point);
    }

    for t in t1..t2 {
        println!("Time {}",t);
        let mut min_x: isize = 1000000;
        let mut max_x: isize = -1000000;
        let mut min_y: isize = 1000000;
        let mut max_y: isize = -1000000;
        let mut graph: HashSet<(isize,isize)> = HashSet::new();
        for point in &points {
            let x:isize = point.start_x + (t as isize) * point.velocity_x;
            let y:isize = point.start_y + (t as isize) * point.velocity_y;
            if x < min_x { 
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y
            {
                max_y =y;
            }
            graph.insert((x,y));
        }    
        println!("{} {} {} {}",min_x,max_x,min_y,max_y);
        for y in min_y..max_y+1 {
            for x in min_x..max_x+1 {
                if graph.contains(&(x,y)) {
                    print!("{}","#");                    
                } else {
                    print!("{}",".");
                }
            }
            print!("{}","\n");
        }
        println!("{}","");
    }
  0
}

    
fn day10b(_filename: &str) -> u32 {    
   0
}

pub fn run() {  
    day10a("input/10.txt",10110,10150);
    println!("Part 2 answer: {:?}", day10b("input/07.txt")); 
}
