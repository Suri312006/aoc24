use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    f32::INFINITY,
    fs,
    ops::Div,
};

// bounds are 50:50
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Cord {
    pub x: isize,
    pub y: isize,
}

const DIM: usize = 50;

fn main() {
    let input = fs::read_to_string("input").expect("input not in root");
    // let input = fs::read_to_string("test").expect("input not in root");
    let mut antennas: HashMap<char, Vec<Cord>> = HashMap::new();

    // first we want to serialize this input into a form thats most useful
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let x = x as isize;
            let y = y as isize;
            if char == '.' {
                continue;
            }
            match antennas.get_mut(&char) {
                Some(locs) => {
                    locs.push(Cord { x, y });
                }
                None => {
                    antennas.insert(char, vec![Cord { x, y }]);
                }
            }
        }
    }

    // ok now we have our map of antennas, lets create a set
    let mut antinodes: HashSet<Cord> = HashSet::new();
    for (_, antennae) in antennas.iter() {
        for uno_ant in antennae {
            antinodes.insert(*uno_ant);
        }
        for pair in antennae.into_iter().combinations(2) {
            // now we have to calculate where the antinodes would be for these two
            // an anti-node occurs at any point that is perfectly in line with the two points but only when one
            // of the points is twice as far as the other.
            let mut left_p = **pair.get(0).unwrap();
            let mut right_p = **pair.get(1).unwrap();

            if right_p.x < left_p.x {
                let tmp = left_p.clone();
                left_p = right_p;
                right_p = tmp;
            }

            let left_p = left_p;
            let right_p = right_p;
            let x: f32 = 5_f32;
            let y = x.div(0_f32);

            let slope = ((right_p.y - left_p.y) as f32) / ((right_p.x - left_p.x) as f32);

            for i in 1..50 {
                let abs_x = (left_p.x - right_p.x).abs() * i;
                let abs_y = (left_p.y - right_p.y).abs() * i;

                let mut antiuno: Option<Cord> = None;
                let mut antidos: Option<Cord> = None;

                match slope {
                    INFINITY => {
                        // vertical line
                        let mut up_p = left_p;
                        let mut down_p = right_p;

                        if down_p.y < up_p.x {
                            let tmp = up_p.clone();
                            up_p = down_p;
                            down_p = tmp;
                        }

                        let up_p = up_p;
                        let down_p = down_p;

                        antiuno = Some(Cord {
                            x: up_p.x,
                            y: up_p.y - abs_y,
                        });

                        antidos = Some(Cord {
                            x: down_p.x,
                            y: down_p.y + abs_y,
                        });
                    }
                    _ => {
                        if slope > 0_f32 {
                            antiuno = Some(Cord {
                                x: left_p.x - abs_x,
                                y: left_p.y - abs_y,
                            });

                            antidos = Some(Cord {
                                x: right_p.x + abs_x,
                                y: right_p.y + abs_y,
                            });
                        } else {
                            antiuno = Some(Cord {
                                x: left_p.x - abs_x,
                                y: left_p.y + abs_y,
                            });

                            antidos = Some(Cord {
                                x: right_p.x + abs_x,
                                y: right_p.y - abs_y,
                            });
                        }
                    }
                }
                // now we have to do the iter?
                antinodes.insert(antiuno.unwrap());
                antinodes.insert(antidos.unwrap());
            }
        }
    }

    let antinodes: Vec<Cord> = antinodes
        .into_iter()
        .filter(|e| e.x >= 0 && e.y >= 0 && e.x < DIM as isize && e.y < DIM as isize)
        .collect();

    println!("total number of antinodes: {}", antinodes.len());
    print_antinodes(antinodes);

    // println!("antinodes: {:#?}", antinodes);
}

fn print_antinodes(anodes: Vec<Cord>) {
    let mut outer = Vec::with_capacity(DIM);
    let mut inner = Vec::with_capacity(DIM);
    inner.resize(DIM, false);
    outer.resize(DIM, inner);
    let mut print_buff: Vec<Vec<bool>> = outer;
    for anode in anodes {
        let x = print_buff
            .get_mut(anode.y as usize)
            .unwrap()
            .get_mut(anode.x as usize)
            .unwrap();
        *x = true;
    }
    for row in print_buff {
        for col in row {
            if col {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
