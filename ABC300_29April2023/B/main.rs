use std::{collections::VecDeque, process::exit};

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [String;h],
        mut b: [String;h],
    }
    for _i in 0..h {
        horizontal_sift(h, &mut a);
        for _j in 0..w {
            vertical_sift(h, &mut a);
            if a == b {
                println!("Yes");
                exit(0);
            }
        }
    }
    println!("No");
}

fn horizontal_sift(h: usize, map: &mut [String]) {
    let zero_keep: String = map[0].clone(); 
    for i in 0..h-1 {
        map[i] = map[i+1].clone();
    }
    map[h-1] = zero_keep;
}

fn vertical_sift(h: usize, map: &mut [String]) {
    for i in 0..h {
        map[i] = line_process(map[i].clone());
    }
}

fn line_process(line: String) -> String{
    let mut charactors: VecDeque<char> = line.chars().collect();
    let tail: char = charactors.pop_back().unwrap();

    charactors.push_front(tail);
    let processed = charactors.iter().collect();
    return processed;
}