use std::fs::read_to_string;

#[derive(Debug)]
enum FreeState {
    Free,
    Unfree(usize),
}

enum NumType {
    BlockSize,
    FreeSpace,
}

fn main() {
    let input = read_to_string("input").expect("should be there");
    // let input = read_to_string("test").expect("should be there");
    let mut disk_map: Vec<usize> = vec![];
    for line in input.lines() {
        line.chars().for_each(|e| {
            disk_map.push(e.to_digit(10).expect("should be num") as usize);
        });
    }

    let mut decomp_disk: Vec<FreeState> = vec![];
    let mut curr_state = NumType::BlockSize;
    let mut id = 0;
    for elem in disk_map.iter() {
        match curr_state {
            NumType::BlockSize => {
                // meaning we have to push into
                for _ in 0..*elem as usize {
                    decomp_disk.push(FreeState::Unfree(id));
                }
                id += 1;
                curr_state = NumType::FreeSpace;
            }
            NumType::FreeSpace => {
                for _ in 0..*elem as usize {
                    decomp_disk.push(FreeState::Free);
                }
                curr_state = NumType::BlockSize;
            }
        }
    }

    // now the hardest part
    // find right most
    // while let Some(i) = free_i(&decomp_disk) {
    //     let (right_i, right_val) = get_last(&decomp_disk).unwrap();
    //     let free_spot = decomp_disk.get_mut(i).unwrap();
    //     *free_spot = FreeState::Unfree(right_val);
    //     let vacated_spot = decomp_disk.get_mut(right_i).unwrap();
    //     *vacated_spot = FreeState::Free;
    // }
    let mut cursor = decomp_disk.len();
    // while cursor > 0 {
    while let Some((right_idxs, right_vals)) = get_last(&decomp_disk, cursor) {
        // println!("{cursor}");
        // print_disk(&decomp_disk);
        // println!("{:#?}", right_vals);
        let free_spots = free_i(
            &decomp_disk,
            right_vals.len(),
            right_idxs.first().unwrap() - 1,
        );
        match free_spots {
            Some(space) => {
                // logic for that
                for (iter, spot) in space.iter().enumerate() {
                    if iter < right_vals.len() {
                        let free_spot = decomp_disk.get_mut(*spot).unwrap();
                        *free_spot = FreeState::Unfree(*right_vals.first().unwrap());
                    }
                }

                for spot in right_idxs.clone() {
                    let vacated_spot = decomp_disk.get_mut(spot).unwrap();
                    *vacated_spot = FreeState::Free;
                }
                // cursor = *right_idxs.last().unwrap();
            }
            None => {
                // println!("{:#?}", right_idxs);
                // println!("reached none");

                cursor = *right_idxs.last().unwrap() - 1;
            }
        }
    }

    let mut total = 0;
    for (i, elem) in decomp_disk.iter().enumerate() {
        match elem {
            FreeState::Free => {}
            FreeState::Unfree(elem) => {
                total += i * elem;
            }
        }
    }
    println!("checksum: {total}");
}

fn free_i(decomp_disk: &Vec<FreeState>, req_size: usize, constr: usize) -> Option<Vec<usize>> {
    let mut free_chunk_idxs: Vec<Vec<usize>> = vec![];

    let mut seen_free = false;

    let mut buffer = vec![];

    for (i, elem) in decomp_disk.iter().enumerate() {
        if i == constr {
            if seen_free {
                // break;
                free_chunk_idxs.push(buffer.clone());
                buffer.clear();
                seen_free = false;
            }
            break;
        }
        match elem {
            FreeState::Free => {
                buffer.push(i);
                seen_free = true;
            }
            FreeState::Unfree(_) => {
                if seen_free {
                    // break;
                    free_chunk_idxs.push(buffer.clone());
                    buffer.clear();
                    seen_free = false;
                }
            }
        }
    }
    // println!("FREE: {:#?}", free_chunk_idxs);

    for chunk in free_chunk_idxs {
        if chunk.len() >= req_size {
            return Some(chunk);
        }
    }

    // if free_chunk_idxs.len() >= req_size {
    //     return Some(free_chunk_idxs); // unwrapping since i should have a value by now
    // }

    None
}

fn get_last(
    decomp_disk: &Vec<FreeState>,
    // target: usize,
    cursor: usize,
) -> Option<(Vec<usize>, Vec<usize>)> {
    // (index, value)
    let mut seen = vec![];
    let mut idxs = vec![];

    for (i, elem) in decomp_disk.iter().enumerate().rev() {
        if i > cursor {
            continue;
        }
        match elem {
            FreeState::Free => {}
            FreeState::Unfree(elem) => {
                if !seen.contains(elem) && !seen.is_empty() {
                    // we are done

                    return Some((idxs, seen));
                } else {
                    seen.push(*elem);
                    idxs.push(i);
                }
                // if *elem != target && seen.is_empty() {
                // } else {
                //     if *elem != target {
                //         return Some((idxs, seen));
                //     } else {
                //         seen.push(*elem);
                //         idxs.push(i);
                //     }
                // }
            }
        }
    }
    None
}

fn last_elem(decomp_disk: &Vec<FreeState>) -> Option<usize> {
    for elem in decomp_disk.iter().rev() {
        match elem {
            FreeState::Free => {}
            FreeState::Unfree(thing) => return Some(*thing),
        }
    }
    None
}

fn print_disk(decomp_disk: &Vec<FreeState>) {
    let mut to_print = String::new();
    for state in decomp_disk {
        match state {
            FreeState::Free => {
                to_print.push('.');
            }
            FreeState::Unfree(elem) => {
                to_print.push_str((*elem).to_string().as_str());
            }
        }
    }
    println!("{to_print}");
}
