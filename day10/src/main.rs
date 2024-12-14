use std::{collections::HashSet, fs};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    row: i32,
    col: i32,
}

type Map = Vec<Vec<u32>>;

fn main() {
    let input = fs::read_to_string("input").expect("input should be in root");
    // let input = fs::read_to_string("test").expect("input should be in root");

    let mut map: Map = vec![];

    let mut zero_locs = vec![];

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let num = char.to_digit(10).unwrap();

            if num == 0 {
                zero_locs.push(Pos {
                    row: row as i32,
                    col: col as i32,
                });
            }

            match map.get_mut(row) {
                Some(row_vec) => {
                    row_vec.push(num);
                }
                None => {
                    map.insert(row, vec![num]);
                }
            }
        }
    }

    let mut total_score = 0;
    let mut rating = 0;

    for zero_pos in zero_locs {
        // let zero_pos = Pos { row: 5, col: 2 };
        let mut peak_locs = HashSet::new();
        find_peaks(zero_pos, &map, &mut peak_locs, &mut rating);
        total_score += peak_locs.len();
    }

    println!("Total Score of Trailheads (part1) {total_score}");
    println!("Total Rating of Trailheads (part2) {rating}");
}

fn find_peaks(curr_pos: Pos, map: &Map, peak_locs: &mut HashSet<Pos>, ret_count: &mut i32) {
    // good hiking trail is
    // starts at height 0
    // ends at height 9
    // always increases by a height of 1
    let curr_val = *map
        .get(curr_pos.row as usize)
        .expect("should be a valid row")
        .get(curr_pos.col as usize)
        .expect("should be valid col");

    if curr_val == 9 {
        *ret_count += 1;
        peak_locs.insert(curr_pos);
        return;
    }

    for neigh in iter_neighbors(curr_pos, map) {
        if *map
            .get(neigh.row as usize)
            .expect("should be a valid row")
            .get(neigh.col as usize)
            .expect("should be valid col")
            == curr_val + 1
        {
            find_peaks(neigh, map, peak_locs, ret_count);
        }
    }
}

fn iter_neighbors(curr_pos: Pos, map: &Map) -> Vec<Pos> {
    let mut neighbors = vec![];
    // if there is a row above us
    if let Some(_) = map.get((curr_pos.row - 1) as usize) {
        // theres always something right above
        neighbors.push(Pos {
            row: curr_pos.row - 1,
            col: curr_pos.col,
        });
    }
    //do the ones on the sides

    // left
    if map
        .get((curr_pos.row) as usize)
        .unwrap()
        .get((curr_pos.col - 1) as usize)
        .is_some()
    {
        neighbors.push(Pos {
            row: curr_pos.row,
            col: curr_pos.col - 1,
        });
    }
    // right
    if map
        .get((curr_pos.row) as usize)
        .unwrap()
        .get((curr_pos.col + 1) as usize)
        .is_some()
    {
        neighbors.push(Pos {
            row: curr_pos.row,
            col: curr_pos.col + 1,
        });
    }

    // if there are ones eside us
    if let Some(_) = map.get((curr_pos.row + 1) as usize) {
        // theres always something right below
        neighbors.push(Pos {
            row: curr_pos.row + 1,
            col: curr_pos.col,
        });
    }

    neighbors
}
