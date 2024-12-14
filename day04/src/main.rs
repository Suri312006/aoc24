use std::fs::read_to_string;

#[derive(Copy, Clone)]
enum Dir {
    UL,
    UC,
    UR,
    L,
    R,
    DL,
    DC,
    DR,
}

impl Dir {
    fn iterator() -> impl Iterator<Item = Dir> {
        [
            Dir::UL,
            Dir::UC,
            Dir::UR,
            Dir::L,
            Dir::R,
            Dir::DL,
            Dir::DC,
            Dir::DR,
        ]
        .iter()
        .copied()
    }
}

#[derive(Copy, Clone)]
struct Cord {
    row: usize,
    col: usize,
}

fn find_char_around(map: &[Vec<char>], target_char: char, start: Cord, dir: Dir) -> Option<Cord> {
    let row = start.row;
    let col = start.col;
    match dir {
        Dir::UL => {
            if *map.get(row - 1)?.get(col - 1)? == target_char {
                return Some(Cord {
                    row: row - 1,
                    col: col - 1,
                });
            }

            None
        }
        Dir::UC => {
            if *map.get(row - 1)?.get(col)? == target_char {
                return Some(Cord { row: row - 1, col });
            }
            None
        }
        Dir::UR => {
            if *map.get(row - 1)?.get(col + 1)? == target_char {
                return Some(Cord {
                    row: row - 1,
                    col: col + 1,
                });
            }
            None
        }
        Dir::L => {
            if *map.get(row)?.get(col - 1)? == target_char {
                return Some(Cord { row, col: col - 1 });
            }
            None
        }
        Dir::R => {
            if *map.get(row)?.get(col + 1)? == target_char {
                return Some(Cord { row, col: col + 1 });
            }
            None
        }
        Dir::DL => {
            if *map.get(row + 1)?.get(col - 1)? == target_char {
                return Some(Cord {
                    row: row + 1,
                    col: col - 1,
                });
            }
            None
        }
        Dir::DC => {
            if *map.get(row + 1)?.get(col)? == target_char {
                return Some(Cord { row: row + 1, col });
            }
            None
        }
        Dir::DR => {
            if *map.get(row + 1)?.get(col + 1)? == target_char {
                return Some(Cord {
                    row: row + 1,
                    col: col + 1,
                });
            }
            None
        }
    }
}

fn determine_xmas(map: &[Vec<char>], start: Cord, dir: Dir) -> Option<()> {
    find_char_around(
        map,
        'S',
        find_char_around(map, 'A', find_char_around(map, 'M', start, dir)?, dir)?,
        dir,
    )
    .map(|_| ())
}

fn count_xmas(map: &[Vec<char>], start: Cord) -> usize {
    let mut found = 0;
    assert_eq!(map[start.row][start.col], 'X');
    for dir in Dir::iterator() {
        let res = determine_xmas(map, start, dir);
        if res.is_some() {
            found += 1;
        }
    }
    found
}

// part2
#[rustfmt::skip]
fn determine_x_mas(map: &[Vec<char>], start: Cord) -> Option<()> {
    if (
           find_char_around(map, 'M', start, Dir::UL).is_some()
        && find_char_around(map, 'M', start, Dir::UR).is_some()
        && find_char_around(map, 'S', start, Dir::DL).is_some()
        && find_char_around(map, 'S', start, Dir::DR).is_some())
        || (
           find_char_around(map, 'M', start, Dir::UL).is_some()
        && find_char_around(map, 'M', start, Dir::DL).is_some()
        && find_char_around(map, 'S', start, Dir::UR).is_some()
        && find_char_around(map, 'S', start, Dir::DR).is_some())
        || (
           find_char_around(map, 'M', start, Dir::DR).is_some()
        && find_char_around(map, 'M', start, Dir::UR).is_some()
        && find_char_around(map, 'S', start, Dir::DL).is_some()
        && find_char_around(map, 'S', start, Dir::UL).is_some())
        || (
           find_char_around(map, 'M', start, Dir::DL).is_some()
        && find_char_around(map, 'M', start, Dir::DR).is_some()
        && find_char_around(map, 'S', start, Dir::UL).is_some()
        && find_char_around(map, 'S', start, Dir::UR).is_some())
    {
        return Some(());
    }

    None
}

fn count_x_mas(map: &[Vec<char>], start: Cord) -> usize {
    let mut found = 0;
    assert_eq!(map[start.row][start.col], 'A');
    if determine_x_mas(map, start).is_some() {
        found += 1;
    }
    found
}

fn main() {
    let input = read_to_string("input").expect("input should be at root");
    // want a versatile 2d input
    let mut map: Vec<Vec<char>> = vec![vec![]];

    input.lines().for_each(|line| {
        let mut line_vec = vec![];
        line.chars().for_each(|char| {
            line_vec.push(char);
        });
        map.push(line_vec);
    });

    // find first x

    let mut total_p1 = 0;
    let mut total_p2 = 0;

    map.iter().enumerate().for_each(|(row, row_vec)| {
        row_vec.iter().enumerate().for_each(|(col, char)| {
            if *char == 'X' {
                let count = count_xmas(&map, Cord { row, col });
                total_p1 += count;
                // we can do something now!
            }

            if *char == 'A' {
                let count = count_x_mas(&map, Cord { row, col });
                total_p2 += count;
            }
        });
    });

    println!("Total XMAS (Part 1): {total_p1}");
    println!("Total X-MAS (Part 2): {total_p2}");
}
