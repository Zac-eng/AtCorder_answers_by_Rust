use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [String;h],
    }
    let cross_range: usize = min(h, w);
    let mut cross_list: Vec<usize> = vec![0;cross_range];
    let map: Vec<Vec<char>> = gen_map_list(h, &c);
    let center_list = find_cross_center(&map);
    for i in 0..center_list.len() {
        let cross_size: usize = check_cross_size(center_list[i], h, w, 1, &map);
        cross_list[cross_size - 1] += 1;
    }
    let strvec: Vec<String> = cross_list.iter().map(|x| x.to_string()).collect();
    let printstr: String = strvec.join(" ");
    println!("{}", printstr);
}

fn gen_map_list(h: usize, map: &[String]) -> Vec<Vec<char>> {
    let mut map_list: Vec<Vec<char>> = vec![Vec::new();h];

    for i in 0..h {
        for elem in (&map[i]).chars() {
            map_list[i].push(elem);
        }
    }
    return map_list;
}

fn find_cross_center(map: &Vec<Vec<char>>) -> Vec<[usize;2]> {
    let mut center_list: Vec<[usize;2]> = Vec::new();

    for i in 1..map.len()-1 {
        for j in 1..map[i].len()-1 {
            if map[i][j] == '#' {
                if judge_center(&map, i, j) == 1 {
                    center_list.push([i, j]);
                }
            }
        }
    }
    return center_list;
}

fn judge_center(map: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut judge: usize = 1;

    if map[x + 1][y + 1] != '#' {judge = 0};
    if map[x + 1][y - 1] != '#' {judge = 0};
    if map[x - 1][y + 1] != '#' {judge = 0};
    if map[x - 1][y - 1] != '#' {judge = 0};
    return judge;
}

fn check_cross_size(pos: [usize;2], h: usize, w: usize, reach: usize, map: &Vec<Vec<char>>) -> usize{
    let edge_top: usize = pos[0] - reach;
    let edge_bottom: usize = pos[0] + reach;
    let edge_left: usize = pos[1] - reach;
    let edge_right: usize = pos[1] + reach;

    if edge_top == 0 || edge_bottom == h-1 || edge_left == 0 || edge_right == w-1 {
        return reach;
    }
    else if map[edge_top-1][edge_left-1] != '#' || map[edge_top-1][edge_right+1] != '#'
                || map[edge_bottom+1][edge_left-1] != '#' || map[edge_bottom+1][edge_right+1] != '#' {
                    return reach;
                }
    else {
        return check_cross_size(pos, h, w, reach + 1, map)
    }
}