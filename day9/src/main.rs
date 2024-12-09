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
    // let input = read_to_string("input").expect("should be there");
    let input = read_to_string("test").expect("should be there");
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

    println!("{:#?}", decomp_disk);
}
