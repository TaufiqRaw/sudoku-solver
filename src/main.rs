mod my_error;
use my_error::MyError;

const SUDOKU_COLS : usize = 9;
const SUDOKU_FRAME : usize = 9*9;
const FILLED : u8 = 255; 

fn main() {
    let mut done_flag = false;
    let mut main_array : [u8; SUDOKU_FRAME] = [
            0, 0, 1, 0, 0, 0, 9, 0, 0,
            0, 7, 0, 0, 0, 8, 4, 3, 0,
            8, 0, 0, 6, 0, 0, 0, 0, 0,
            0, 0, 2, 0, 1, 0, 0, 0, 0,
            0, 4, 0, 0, 0, 6, 8, 7, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 5,
            0, 0, 4, 2, 0, 0, 3, 5, 0,
            0, 5, 0, 0, 0, 0, 0, 0, 6,
            0, 0, 0, 0, 0, 3, 0, 0, 9,
        ];

    let row_potentials = get_row_potentials(&mut main_array, 0).expect("cant find first row potential, aborting operation");
    let most_constrained_cols = get_min_index(&row_potentials);

    solve(&mut main_array, most_constrained_cols, &mut done_flag);

    for i in 0 .. 9{
        println!("{:?}", &main_array[0 + (i * 9) .. 9 + (i * 9)]);
    }
}

fn solve(array : &mut [u8], position : usize, done_flag : &mut bool){
    let row: usize = (position as f64 / 9.0).floor() as usize;
    let candidate = match get_candidate(array, position) {
        Ok(candidate) => candidate,
        _ => return,
    };

    if *done_flag {
        return;
    }

    
    for item in &candidate {
        array[position] = *item;

        if *done_flag {
            return;
        }

        let row_potentials = get_row_potentials(array, row);
        let row_potentials = match row_potentials {
            Err(_) => {
                array[position] = 0;
                return ();
            },
            Ok(data) => data
        };
        let most_constrained_cols = get_min_index(&row_potentials);

        //TODO: not use index 1 on new row
        if is_filled(&array[0 + (row * 9) .. 9 + (row * 9)]) {
            if row+1 >= SUDOKU_COLS {
                *done_flag = true;
                return ();
            }
            
            let row_potentials = get_row_potentials(array, row+1);
            let row_potentials = match row_potentials {
                Err(_) => {
                    if *done_flag {
                        return;
                    }
                    array[position] = 0;
                    return;
                },
                Ok(data) => data
            };

            let arr_pos = (row+1) * SUDOKU_COLS + get_min_index(&row_potentials);
            solve(array, arr_pos, done_flag);

            if *done_flag {
                return;
            }
            
            //clean after backtrack fail;
            array[arr_pos] = 0;
        }else {
            let arr_pos = row * SUDOKU_COLS + most_constrained_cols;
            solve(array, arr_pos, done_flag);

            if *done_flag {
                return;
            }
            
            //clean after backtrack fail;
            array[arr_pos] = 0;
        }
    }
}

fn is_filled(array : &[u8])-> bool{
    for i in array{
        if *i == 0 {
            return false;
        }
    };

    true
}

fn get_row_potentials(sudoku_array : &mut [u8], row : usize)-> Result<[u8; SUDOKU_COLS], MyError> {
    let mut row_potentials: [u8; SUDOKU_COLS] = [0; SUDOKU_COLS];
    for col in 0 .. SUDOKU_COLS{
        if sudoku_array[get_index(col, row)] > 0{
            row_potentials[col] = FILLED;
            continue;
        }
        let candidate = get_candidate(sudoku_array, get_index(col, row))?;
        row_potentials[col] = candidate.len() as u8;
    };

    Ok(row_potentials)
}

fn get_min_index(array : &[u8])-> usize {
    let mut min: usize = 0;
    for (i, value) in array.iter().enumerate() {
        if value < &array[min] {
            min = i;
        }
    }
    min
}

//position take real 1 dimensional index
fn get_candidate(sudoku_array : &mut [u8], position: usize)-> Result<Vec<u8>, MyError> {
    let row: usize = (position as f64 / 9.0).floor() as usize;
    let col: usize = position as usize % SUDOKU_COLS as usize;
    let sector: usize = ((col as f32 / 3.0).floor() as usize + 1) + (((row) as f32 / 3.0).floor() as usize)*3;
    let mut candidate: Vec<u8> = Vec::new();

    //TODO : fix the nested for, must not be nested;
    for i in 1 .. 10 {
        let mut found_same = false;
        //col constraint check
        for col in 0 .. SUDOKU_COLS {
            if sudoku_array[get_index(col, row)] == i {
                found_same = true;
            };
        };

        //row constraint check
        if !found_same {
            for row in 0 .. SUDOKU_COLS {
                if sudoku_array[get_index(col, row)] == i {
                    found_same = true;
                };
            };
        }

        //sector constraint check
        if !found_same {
            for col in 0 .. SUDOKU_COLS {
                if sudoku_array[((col % 3)+(((sector-1) % 3) * 3)) + 
                                    ((((col as f32 / 3.0).floor() as usize) * SUDOKU_COLS) + 
                                        (((sector-1) as f32 / 3.0).floor() as usize * 3 * SUDOKU_COLS))] == i {
                    found_same = true;
                }
            }
        }

        //push if no same number
        if !found_same {
            candidate.push(i);
        }
    };
    

    match candidate.len() {
        0 => Err(MyError::new("Cant find new candidate")),
        _ => Ok(candidate)
    }
}

//row and col are 0 index based (so real row 1 is written as 0 here)
fn get_index(col: usize, row: usize)-> usize {
    col + (row * SUDOKU_COLS ) 
}