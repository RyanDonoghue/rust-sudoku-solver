mod sudoku;
use std::io::{stdout, Write};
use std::time::{Instant};
use crossterm::{
    cursor, terminal, Result, ExecutableCommand
};
use sudoku::{SudokuBoard};
use quick_protobuf::{BytesReader};
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

const PRINTING_ENABLED: bool = false ;
const PRINT_DELAY: u64 = 50;

fn main(){

    println!("Beginning polling for messages ...");

    let mut consumer =
    Consumer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_topic_partitions("sudoku-boards".to_owned(), &[0])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("my-group".to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let read_message = {
                    let mut reader = BytesReader::from_bytes(&m.value);
                    reader
                       .read_message::<SudokuBoard>(&m.value)
                       .expect("Cannot read message")
                };
            
                print_type_of(&read_message);

                let mut board_to_solve: Vec<Vec<i32>> = Vec::with_capacity(9 * 9);
                for i in 0..9 {
                    let row_vec: Vec<i32> = read_message.rows[i].row.clone();
                    board_to_solve.push(row_vec);
                }

                let _ = clear_console();
                println!("Solving puzzle.");
                let start = Instant::now();
                let solved_board: &Vec<Vec<i32>> = solve(&mut board_to_solve);
                let duration = start.elapsed();
                let _ = print_grid(&solved_board);
                println!("Sudoku puzzle solved, took {:?}", duration);
                for y in 0..9 {
                    println!("{:?}", solved_board[y]);
                }
            }
            let _ = consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }

}

fn solve(mut board: &mut Vec<Vec<i32>>) -> &mut Vec<Vec<i32>> {
    if is_solved(board) {
        return board
    }

    for y in 0..9 as usize {
        for x in 0..9 as usize {
            if board[y][x] == 0 {
                for n in 1..10 as i32 {
                        if is_valid_pos(y, x, n, board) {
                            board[y][x] = n;
                            let _ = print_grid(board);
                            board = solve(board);
                            if is_solved(board) {
                                return board
                            }
                            board[y][x] = 0;
                        }
                }
                return board
            }
        }
    };
    return board;
    
}

fn is_valid_pos(y: usize, x: usize, n: i32, board: &Vec<Vec<i32>>) -> bool {
    //check horizontal
    for y_to_check in 0..9 as usize {
        if board[y_to_check][x as usize] == n {
            return false
        }
    
    }
    //check vertical
    for x_to_check in 0..9 {
        if board[y as usize][x_to_check] == n {
            return false
        }
    }
    //check box
    let block_coords: (usize, usize) = get_block_first_position(y, x);
    for x_to_check in block_coords.0..block_coords.0 + 2{
        for y_tocheck in block_coords.1..block_coords.1 + 2 {
            if  board[x_to_check as usize][y_tocheck as usize] == n {
                return false
            }
        }
    }
    true
}

fn get_block_first_position(y: usize, x: usize) -> (usize, usize) {
    let start_y: usize;
    if y > 5 {
        start_y = 6
    } else if y > 2 {
        start_y = 3;
    } else {
        start_y = 0;
    };
    let start_x: usize;
    if x > 5 {
        start_x = 6
    } else if x > 2 {
        start_x = 3;
    } else {
        start_x = 0;
    };
    (start_y, start_x)
} 

fn is_solved(board: &Vec<Vec<i32>>) -> bool {
    for y in 0..9 as usize {
        for x in 0..9 as usize {
            if board[y][x] == 0 {
                return false
            }
        }
    }
    true
}

fn print_grid(board: &Vec<Vec<i32>>) -> Result<()> {
    if PRINTING_ENABLED {
        let mut stdout = stdout();
        stdout.execute(cursor::MoveUp(9))?;
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
        for y in 0..9 as usize {
            writeln!(stdout, "{:?}", board[y])?;
        }
        std::thread::sleep(std::time::Duration::from_micros(PRINT_DELAY));
    }
    Ok(())
}

fn clear_console() -> Result<()> {
    if PRINTING_ENABLED {
        let mut stdout = stdout();
        for _i in 0..9 {
            writeln!(stdout, "")?;
        }
    }
    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("Read message of type {}", std::any::type_name::<T>())
}