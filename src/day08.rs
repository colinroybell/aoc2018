use std::fs::File;
use std::io::{BufRead,BufReader};
//use regex::Regex;
//use std::collections::{HashSet,HashMap};
use std::collections::VecDeque;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day8a() {
        assert_eq!(138,day8a("input/08_test1.txt"));
    }

    #[test]
    fn test_day8b() {
        assert_eq!(66,day8b("input/08_test1.txt"));
    }
}   

// code copied from Stackoverflow here
struct Node {
    children: Vec<Node>,
    data: Vec<usize>,
    cache: isize,
}

impl Node {

    pub fn new() -> Node {
        Node {
            children: vec!(),
            data: vec!(),
            cache: -1
        }
    }

    pub fn expand(&mut self) {
        self.children = vec!(Node::new(), Node::new());
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    fn construct_from_deque(&mut self, items: &mut VecDeque<usize>) {
        let c = items.pop_front().unwrap();
        let d = items.pop_front().unwrap();
        println!("{} children",c);
        for _ in 0..c {
            let mut child = Node::new();
            child.construct_from_deque(items);
            self.children.push(child);
            
        }
        for _ in 0..d {
            let mut datum = items.pop_front().unwrap();
            println!("{} pushing",datum);
            self.data.push(datum);
        }
    }

    fn sum_metadata(&mut self) -> usize {
        let mut sum: usize = 0;
        for mut child in &mut self.children {
            println!("{}","in loop");
            sum += child.sum_metadata();
        }
        for datum in &mut self.data {

            sum += *datum;
            println!("{} {}",datum,sum);
        }
        sum
    }

    fn sum_metadata2(&mut self) -> usize {
        println!("{}","start");
        if self.cache > -1 {
            println!("case 1:{}",self.cache);
            return self.cache as usize
        } else if self.children.len() == 0 { 
            self.cache = self.sum_metadata() as isize;
            println!("case 2:{}",self.cache);
            return self.cache as usize;
        } else {
            let mut sum: usize = 0;
            for datum in &mut self.data {
                let datum = *datum;
                println!("{} {}",datum,self.children.len());
                if datum <= self.children.len() {
                    sum += self.children[datum-1].sum_metadata2();
                }
            }
            self.cache = sum as isize;
            println!("case 3:{}",self.cache);
            return sum; 
        }    
    }
}


fn day8a(filename: &str) -> usize {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
  
    let mut items: VecDeque<usize> = VecDeque::new();
   
    let mut text :String = String::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        text.push_str(&line);
    }

    println!("{} text",text);

    for t in text.split(' ') {
        let n = t.parse::<usize>().ok().unwrap();
        println!("{} found",n);
        items.push_back(n);
    }
 
    let mut head :Node = Node::new();

    head.construct_from_deque(&mut items);

    head.sum_metadata()
} 
    
fn day8b(filename: &str) -> usize {    
   let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
  
    let mut items: VecDeque<usize> = VecDeque::new();
   
    let mut text :String = String::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        text.push_str(&line);
    }

    for t in text.split(' ') {
        let n = t.parse::<usize>().ok().unwrap();
        items.push_back(n);
    }
 
    let mut head :Node = Node::new();

    head.construct_from_deque(&mut items);

    head.sum_metadata2()
}

pub fn run() {   
  
//  println!("Part 1 answer: {:?}", day8a("input/08.txt"));
  println!("Part 2 answer: {:?}", day8b("input/08.txt")); 
}
