use std::env;
use std::fmt;
use std::fs;

#[derive(Debug, Copy, Clone)]
struct Space {
    value: u8,
    called: bool,
}

#[derive(Debug, Copy, Clone)]
struct Board([[Space; 5]; 5]); // board[row][column]

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.0 {
            for cell in row {
                let mut taken = ' ';
                if cell.called {
                    taken = '*';
                }
                if cell.value < 10 {
                    write!(f, " {}{}{} ", taken, cell.value, taken)?;
                } else {
                    write!(f, "{}{}{} ", taken, cell.value, taken)?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl Board {
    fn apply_move(&mut self, number: u8) -> bool {
        let mut changed = false;
        // This seems to not work. Perhaps one of the for loops makes a copy??
        // for row in self.0 {
        //     for mut cell in row {
        //         if cell.value == number && !cell.called {
        //             changed = true;
        //             cell.called = true;
        //             println!("updated {} {} {}", cell.called, cell.value, self);
        //         }
        //     }
        // }
        for row in 0..5 {
            for column in 0..5 {
                if self.0[row][column].value == number {
                    self.0[row][column].called = true;
                    changed = true;
                }
            }
        }
        return changed;
    }

    fn score(&self, last_number: u8) -> u32 {
        let mut score = 0;
        for row in self.0 {
            for cell in row {
                if !cell.called {
                    score += cell.value as u32; // https://stackoverflow.com/a/44552464
                    println!("{}", score);
                }
            }
        }
        score *= last_number as u32;
        return score;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // no additional args; print help text
        eprintln!("Usage: {} infile.txt", args[0]);
        return;
    }

    let filename = &args[1];
    let raw_inputs = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut inputs = raw_inputs.trim().split("\n\n");
    let number_strings = inputs.next().unwrap();
    let board_strings = inputs.collect::<Vec<&str>>();

    let mut boards: Vec<Board> = Vec::new();
    for board_string in board_strings {
        boards.push(parse_board(board_string));
        println!("{}", boards[boards.len() - 1]);
    }

    let mut winning_board = None;
    let mut number = 0; // whatever, just so it's always initialized. It'll get overwritten in the next step
    for number_string in number_strings.split(',') {
        number = number_string.parse().unwrap();
        for i in 0..boards.len() {
            boards[i].apply_move(number);
            println!("{}", boards[i]);
            if check_win(boards[i]) {
                winning_board = Some(boards[i]);
                break;
            }
        }
        if winning_board.is_some() { // Don't call the rest of the numbers if someone has already won
            break;
        }
    }

    let winning_board = winning_board.expect("No board has won?");
    println!("Won:\n{}\nScore: {}", winning_board, winning_board.score(number));
}

fn parse_board(input: &str) -> Board {
    let mut board = Board(
        [[Space {
            value: 0,
            called: false,
        }; 5]; 5],
    );
    for (i, row) in input.split("\n").enumerate() {
        for (j, value) in row.split_whitespace().enumerate() {
            board.0[i][j].value = value.parse().unwrap();
        }
    }
    return board;
}

fn check_win(board: Board) -> bool {
    for row in board.0 {
        if row[0].called && row[1].called && row[2].called && row[3].called && row[4].called {
            return true;
        }
    }
    for column in 0..5 {
        if board.0[0][column].called
            && board.0[1][column].called
            && board.0[2][column].called
            && board.0[3][column].called
            && board.0[4][column].called
        {
            return true;
        }
    }
    return false;
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const RAW_MOVES: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";
    const RAW_BOARDS: &str = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_parse_board() {
        let input = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";
        let board = parse_board(input);
        assert_eq!(board.0[0][0].value, 22);
        assert_eq!(board.0[0][1].value, 13);
        assert_eq!(board.0[0][2].value, 17);
        assert_eq!(board.0[0][3].value, 11);
        assert_eq!(board.0[0][4].value, 0);
        assert_eq!(board.0[1][1].value, 2);
        assert_eq!(board.0[2][2].value, 14);
        assert_eq!(board.0[3][3].value, 18);
        assert_eq!(board.0[4][4].value, 19);
    }

    #[test]
    fn test_check_not_won() {
        let input = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";
        let mut board = parse_board(input);
        board.0[1][0].called = true;
        board.0[1][1].called = true;
        board.0[1][2].called = true;
        board.0[1][3].called = true;
        assert_eq!(check_win(board), false);
    }

    #[test]
    fn test_check_win_row() {
        let input = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";
        let mut board = parse_board(input);
        board.0[1][0].called = true;
        board.0[1][1].called = true;
        board.0[1][2].called = true;
        board.0[1][3].called = true;
        board.0[1][4].called = true;
        assert_eq!(check_win(board), true);
    }

    #[test]
    fn test_check_win_column() {
        let input = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";
        let mut board = parse_board(input);
        board.0[0][1].called = true;
        board.0[1][1].called = true;
        board.0[2][1].called = true;
        board.0[3][1].called = true;
        board.0[4][1].called = true;
        assert_eq!(check_win(board), true);
    }

    #[test]
    fn test_run_game() {
        let mut boards: Vec<Board> = Vec::new();
        for board_string in RAW_BOARDS.split("\n\n") {
            boards.push(parse_board(board_string));
            println!("{}", boards[boards.len() - 1]);
        }
    
        let mut winning_board = None;
        let mut number = 0;
        for number_string in RAW_MOVES.split(',') {
            number = number_string.parse().unwrap();
            for i in 0..boards.len() {
                boards[i].apply_move(number);
                println!("{}", boards[i]);
                if check_win(boards[i]) {
                    winning_board = Some(boards[i]);
                    break;
                }
            }
            if winning_board.is_some() { // Don't call the rest of the numbers if someone has already won
                break;
            }
        }

        let winning_board = winning_board.expect("No board has won?");
        assert_eq!(winning_board.score(number), 4512);
    }
}
