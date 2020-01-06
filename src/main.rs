fn main() {
    println!("Hello, world!");
    let mut board: Vec<Vec<u8>> = vec![];
    board.push(vec![0, 0, 0, 0, 0, 7, 0, 4, 0]);
    board.push(vec![2, 0, 0, 0, 0, 8, 0, 1, 0]);
    board.push(vec![4, 0, 9, 0, 5, 0, 0, 0, 0]);
    board.push(vec![0, 0, 0, 0, 9, 0, 3, 0, 0]);
    board.push(vec![0, 3, 5, 0, 0, 0, 6, 8, 0]);
    board.push(vec![0, 0, 6, 0, 1, 0, 0, 0, 0]);
    board.push(vec![0, 0, 0, 0, 4, 0, 1, 0, 7]);
    board.push(vec![0, 5, 0, 7, 0, 0, 0, 0, 6]);
    board.push(vec![0, 4, 0, 5, 0, 0, 0, 0, 0]);
    for x in board.iter() {
        for y in x.iter() {
            print!("{} ", y);
        }
        print!("\n");
    }
    solve(&mut board);
    println!();
    for x in board.iter() {
        for y in x.iter() {
            print!("{} ", y);
        }
        print!("\n");
    }
}

fn solve(board: &mut Vec<Vec<u8>>) -> bool {
    let mut tracker: (u8, u8) = (0, 0);
    match find_empty(board, &mut tracker) {
        Some(_) => {
            for x in 1..10 {
                if check_location(board, tracker.1, tracker.0, x as u8) {
                    match board.get_mut(tracker.0 as usize) {
                        Some(row) => match row.get_mut(tracker.1 as usize) {
                            Some(slot) => {
                                *slot = x as u8;
                                let flag = solve(board);
                                if flag {
                                    return true;
                                }
                                match board.get_mut(tracker.0 as usize) {
                                    Some(row) => match row.get_mut(tracker.1 as usize) {
                                        Some(slot) => {
                                            *slot = 0 as u8;
                                        }
                                        None => return false,
                                    },
                                    None => return false,
                                }
                            }
                            None => return false,
                        },
                        None => return false,
                    }
                }
            }
        }
        None => return true,
    };
    false
}

fn find_empty(board: &mut Vec<Vec<u8>>, tracker: &mut (u8, u8)) -> Option<()> {
    for (i, x) in board.iter().enumerate() {
        for (i2, y) in x.iter().enumerate() {
            if *y == 0 {
                tracker.0 = i as u8;
                tracker.1 = i2 as u8;
                return Some(());
            }
        }
    }
    None
}

fn check_location(board: &mut Vec<Vec<u8>>, col: u8, row: u8, num: u8) -> bool {
    let col_flag = !check_col(board, col, num);
    let row_flag = !check_row(board, row, num);
    let box_flag = !check_box(board, col - col % 3, row - row % 3, num);
    return col_flag && row_flag && box_flag;
}

fn check_col(board: &mut Vec<Vec<u8>>, col: u8, num: u8) -> bool {
    for x in board.iter() {
        if *(x.get(col as usize).unwrap()) == num {
            return true;
        }
    }
    false
}
fn check_row(board: &mut Vec<Vec<u8>>, row: u8, num: u8) -> bool {
    for x in board.get(row as usize).unwrap().iter() {
        if *x == num {
            return true;
        }
    }
    false
}
fn check_box(board: &mut Vec<Vec<u8>>, col: u8, row: u8, num: u8) -> bool {
    for x in 0..3 {
        for j in 0..3 {
            if *(board
                .get((row + x) as usize)
                .unwrap()
                .get((col + j) as usize)
                .unwrap())
                == num
            {
                return true;
            }
        }
    }
    false
}
