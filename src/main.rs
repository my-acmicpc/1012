use std::{io, usize};

fn solution(w: usize, h: usize, map: &mut Vec<Vec<bool>>) -> u32 {
    fn find_first_non_empty(w: usize, h: usize, map: &Vec<Vec<bool>>) -> Option<(usize, usize)> {
        for y in 0..h {
            for x in 0..w {
                if map[y][x] {
                    return Some((x, y));
                }
            }
        }
        return None;
    }

    fn flood_fill(w: usize, h: usize, x: usize, y: usize, map: &mut Vec<Vec<bool>>) -> () {
        map[y][x] = false;

        if x != 0 && map[y][x - 1] {
            flood_fill(w, h, x - 1, y, map)
        }
        if y != 0 && map[y - 1][x] {
            flood_fill(w, h, x, y - 1, map)
        }
        if x != w - 1 && map[y][x + 1] {
            flood_fill(w, h, x + 1, y, map)
        }
        if y != h - 1 && map[y + 1][x] {
            flood_fill(w, h, x, y + 1, map)
        }
    }

    let mut result = 0u32;

    while let Some((x, y)) = find_first_non_empty(w, h, map) {
        flood_fill(w, h, x, y, map);
        result += 1
    }
    return result;
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let test_case_count = line.trim().parse::<usize>().unwrap();
    for _ in 0..test_case_count {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.trim().split(' ').flat_map(&str::parse::<usize>);
        let w = iter.next().unwrap();
        let h = iter.next().unwrap();
        let c = iter.next().unwrap();
        let mut map = vec![vec![false; w]; h];
        for _ in 0..c {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.trim().split(' ').flat_map(&str::parse::<usize>);
            let x = iter.next().unwrap();
            let y = iter.next().unwrap();
            map[y][x] = true;
        }

        println!("{}", solution(w, h, &mut map));
    }
}
