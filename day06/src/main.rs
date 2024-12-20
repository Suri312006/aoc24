use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Pixel {
    Obstacle,
    Visited,
    NotVisited,
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

impl Dir {
    fn next_pixel(&self, curr_pos: &mut Pos, map: &mut Vec<Vec<Pixel>>) -> Option<Pixel> {
        match *self {
            Dir::Up => curr_pos.y -= 1,
            Dir::Down => curr_pos.y += 1,
            Dir::Left => curr_pos.x -= 1,
            Dir::Right => curr_pos.x += 1,
        };
        // println!("x:{}, y:{}, dir:{:#?}", curr_pos.x, curr_pos.y, self);
        Some(*map.get(curr_pos.y as usize)?.get(curr_pos.x as usize)?)
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("input should be in root");
    let mut map: Vec<Vec<Pixel>> = vec![];

    let mut parsed_pos = None;
    let mut curr_dir = Dir::Up;

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            match char {
                '^' => {
                    // found our starting
                    parsed_pos = Some(Pos::new(j as i32, i as i32));
                    map.get_mut(i).expect("should be here").push(Pixel::Visited);
                }
                '.' => match map.get_mut(i) {
                    Some(vec) => vec.push(Pixel::NotVisited),
                    None => map.push(vec![Pixel::NotVisited]),
                },
                '#' => match map.get_mut(i) {
                    Some(vec) => vec.push(Pixel::Obstacle),
                    None => map.push(vec![Pixel::Obstacle]),
                },
                _ => {}
            }
        }
    }

    let mut curr_pos = parsed_pos.unwrap();
    let mut total = 1;

    calculate_movement(&mut map, &mut curr_pos, &mut curr_dir, &mut total, &mut 0);
    println!("Total visited (part 1): {total}");

    let mut loopers = 0;
    for i in 0..130usize.pow(2) {
        let mut new_map = map.clone();
        let obs_pos = Pos::new((i % 130) as i32, (i / 130) as i32);
        let mut curr_pos = parsed_pos.unwrap();
        let mut curr_dir = Dir::Up;
        match new_map.get_mut(obs_pos.y as usize) {
            Some(v) => match v.get_mut(obs_pos.x as usize) {
                Some(x) => *x = Pixel::Obstacle,
                None => {
                    println!("broke at this obs_pos: {:#?}", obs_pos);
                    break;
                }
            },
            None => {
                println!("broke at this obs_pos: {:#?}", obs_pos);
                break;
            }
        };

        let mut curr_total = 1;
        let mut curr_depth = 1;
        calculate_movement(
            &mut new_map,
            &mut curr_pos,
            &mut curr_dir,
            &mut curr_total,
            &mut curr_depth,
        );

        if curr_depth > 130usize.pow(2) {
            // this one looped
            loopers += 1;
        }
    }

    println!("Total Possible Obstacles (part 2): {loopers}");
}

fn calculate_movement(
    map: &mut Vec<Vec<Pixel>>,
    curr_pos: &mut Pos,
    curr_dir: &mut Dir,
    curr_total: &mut usize,
    curr_depth: &mut usize,
) {
    if *curr_depth > 130usize.pow(2) {
        return;
    }
    // should terminate
    if let Some(pix) = curr_dir.next_pixel(curr_pos, map) {
        match pix {
            Pixel::Visited => {}
            Pixel::NotVisited => {
                let x = map
                    .get_mut(curr_pos.y as usize)
                    .unwrap()
                    .get_mut(curr_pos.x as usize)
                    .unwrap();
                *x = Pixel::Visited;
                *curr_total += 1;
            }
            Pixel::Obstacle => {
                // need to move him back && rotate him right
                match curr_dir {
                    Dir::Up => {
                        curr_pos.y += 1;
                        *curr_dir = Dir::Right;
                    }
                    Dir::Down => {
                        curr_pos.y -= 1;
                        *curr_dir = Dir::Left;
                    }
                    Dir::Left => {
                        curr_pos.x += 1;
                        *curr_dir = Dir::Up;
                    }
                    Dir::Right => {
                        curr_pos.x -= 1;
                        *curr_dir = Dir::Down
                    }
                }
                // need to rotate them right
            }
        }
        *curr_depth += 1;
        // println!("curr depth: {curr_depth}");
        calculate_movement(map, curr_pos, curr_dir, curr_total, curr_depth);
    }
}
