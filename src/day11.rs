//use std::fs::File;
//use std::io::{BufRead,BufReader};
//use regex::Regex;
//use std::collections::{HashSet,HashMap};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day11a() {
        assert_eq!(4,compute_power(8,3,5));
        assert_eq!(-5,compute_power(57,122,79));
        assert_eq!(0,compute_power(39,217,196));
        assert_eq!(4,compute_power(71,101,153));

        assert_eq!((33,45),day11a(18));
    }

    #[test]
    fn test_day11b() {
        assert_eq!(day11b(18),(90,269,16));
        assert_eq!(day11b(42),(232,251,12));
    }
}   

fn compute_power(serial: usize, x: usize, y: usize) -> isize {
    let rack_id = x + 10 ;
    (((((rack_id * y + serial) * rack_id)/100) % 10) as isize)- 5
}

fn day11a(serial: usize) -> (usize,usize) {
    let mut power: [[isize; 300];300] = [[0;300];300];

    for i in 0..300 {
        for j in 0..300 {
            let x = i + 1;
            let y = j + 1;          
            power[i][j] = compute_power(serial, x, y);
        } 
    }
    let mut maximum = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for i in 0..298 {
        for j in 0..298 {
            let mut sum = 0;
            for ii in 0..3 {
                for jj in 0..3 {
                    sum += power[i+ii][j+jj];
                }
            }
            if sum > maximum {
                maximum = sum;
                max_x = i+1;
                max_y = j+1;
            }  
            
        }
    }
    (max_x,max_y)   
}

    
fn day11b(serial: usize) -> (usize,usize,usize) {
    let mut power: [[isize;300];300] = [[0;300];300];
    let mut power_sum: [[isize;301];301] = [[0;301];301];
    for i in 0..300 {
        let mut colSum = 0;
        for j in 0..300 {
            let x = i + 1;
            let y = j + 1;          
            power[i][j] = compute_power(serial, x, y);
            colSum += power[i][j];
            power_sum[i+1][j+1] = colSum + power_sum[i][j+1];
        } 
    }

    let mut maximum = 0;
    let mut max_pos = (0,0,0);
    for size in 1..301 {
        for i in 0..301-size {
            for j in 0..301-size {
                let score = power_sum[i+size][j+size] - power_sum[i][j+size] - power_sum[i+size][j] + power_sum[i][j];
                if score > maximum {
                    maximum = score;
                    max_pos = (i+1, j+1, size);
                }
            }
        }
    }

    max_pos   
}

pub fn run() {  
    println!("Part 1 answer: {:?}", day11a(7165));
    println!("Part 2 answer: {:?}", day11b(7165));
}
