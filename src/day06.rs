use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use std::collections::{HashSet,HashMap};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day6a() {
        assert_eq!(17,day6a("input/06a_test1.txt"));
    }

    #[test]
    fn test_day6b() {
        assert_eq!(16,day6b("input/06a_test1.txt",32));
    }
}   

fn try_new(next_claims: &mut HashMap<(usize,usize),usize>, claims: &mut HashMap<(usize,usize),usize>, n: usize, contested: usize, x: usize, y: usize) -> () {
    let key = (x,y);
    if claims.contains_key(&key) {
        return
    }
    let claim = next_claims.entry(key).or_insert(n);
    if *claim != n {
        *claim = contested;
    }
}

fn day6a(filename: &str) -> usize {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
 
    let re1 = Regex::new(r"(\d+),\s+(\d+)").unwrap();
    
    let mut min_x=1000000;
    let mut min_y=1000000;
    let mut max_x=0;
    let mut max_y=0;

    let mut infinite: HashSet<usize> = HashSet::new();
    let mut claims: HashMap<(usize,usize),usize> = HashMap::new();
    let mut new_claims: HashMap<(usize,usize),usize> = HashMap::new();
    let mut next_claims: HashMap<(usize,usize),usize> = HashMap::new();
    let mut counts:HashMap<usize,usize> = HashMap::new();

    let mut num = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        assert!(re1.is_match(&line));
        let cap = re1.captures(&line).unwrap();
        let x = cap[1].parse().unwrap();
        let y = cap[2].parse().unwrap();     
        new_claims.insert((x,y),num);
        counts.insert(num,0);
        if x < min_x {
            min_x = x
        } 
        if x > max_x {
            max_x = x
        }
        if y < min_y {
            min_y = y
        }
        if y > max_y {
            max_y = y
        }
        num+=1
     }

     
     while new_claims.len() > 0 {
        for (pos,n) in new_claims.iter() {
            let n = *n;
            claims.insert(*pos,n);
            let val = counts.entry(n).or_insert(0);
            *val += 1;
            if pos.0 == min_x || pos.0 == max_x || 
                pos.1 == min_y || pos.1 == max_y {
                infinite.insert(n);
            }
        }
        for (pos,n) in new_claims.iter() {
            let n = *n;
            // FIXME: claims is not being changed but can't see a way to pass otherwise
            if pos.0 < max_x {
                try_new(&mut next_claims,&mut claims,n,num,pos.0+1,pos.1+0);
            }
            if pos.0 > min_x {
                try_new(&mut next_claims,&mut claims,n,num,pos.0-1,pos.1+0);
            }
            if pos.1 < max_y {
                try_new(&mut next_claims,&mut claims,n,num,pos.0+0,pos.1+1);
            }
            if pos.1 > min_y {
                try_new(&mut next_claims,&mut claims,n,num,pos.0+0,pos.1-1);
            }
        }
        new_claims.clear();
        for (pos,n) in next_claims.iter() {
            new_claims.insert(*pos,*n);
        }
        next_claims.clear();
    }
    let mut max_size: usize = 0;
    for n in 0..num {
        if !infinite.contains(&n) {
            let size = counts.get(&n).unwrap();
            if *size > max_size {
                max_size = *size;
            }
        }
    }
    max_size

}

    
fn day6b(filename: &str, threshold: usize) -> usize {    
      let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
 
     let mut cores :Vec<(isize,isize)> = Vec::new();

     let re1 = Regex::new(r"(\d+),\s+(\d+)").unwrap();
   

     for line in f.lines() {
        let line = line.expect("Unable to read line");
        assert!(re1.is_match(&line));
        let cap = re1.captures(&line).unwrap();
        let x = cap[1].parse().unwrap();
        let y = cap[2].parse().unwrap();     
        cores.push((x,y));
     }

     let mut count :usize = 0;
     for x in 0..400 {
        for y in 0..400 {
            let mut sum :usize = 0;
            for core in &cores {
                sum+=((x-core.0).abs()+(y-core.1).abs()) as usize;
            }
            if sum < threshold {
                count+=1
            }
        }
     }
     count

}

pub fn run() {  
    println!("Part 1 answer: {:?}", day6a("input/06.txt"));
    println!("Part 2 answer: {:?}", day6b("input/06.txt",10000)); 
}
