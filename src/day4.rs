use board::Board;
use day_setup::Utils;

use crate::utils::day_setup;

pub fn run() {
    // run_part(function, part_num, day_num)
    Utils::run_part(part1, 1, 4, 35711);
    Utils::run_part(part2, 2, 4, 5586);
}

fn part1(mut input: Vec<String>) -> u64 {
    let (nums_to_draw, mut boards) = pre_processing(&mut input);

    for nums in nums_to_draw.chunks(5) {
        for num in nums {
            let found_board = boards
                .iter_mut()
                .filter_map(|board| {
                    board.mark_on_board(*num);
                    if board.is_winner {
                        Some(board)
                    } else {
                        None
                    }
                })
                .collect::<Vec<&mut Board>>();

            if let Some(found_board) = found_board.first() {
                return found_board.sum_board_elem() * (*num as u64);
            }
        }
    }

    unreachable!("At least one board must win")
}

fn part2(mut input: Vec<String>) -> u64 {
    let (nums_to_draw, mut boards) = pre_processing(&mut input);

    let mut last_board = None;
    let mut last_winning_num = None;
    for nums in nums_to_draw.chunks(5) {
        for num in nums {
            let found_board = boards
                .iter_mut()
                .enumerate()
                .filter_map(|(idx, board)| {
                    board.mark_on_board(*num);
                    if board.is_winner {
                        Some((idx, board))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, &mut Board)>>();

            if !found_board.is_empty() {
                last_winning_num = Some(*num);
                let indexes = found_board.iter().map(|(b, _)| *b).collect::<Vec<usize>>();
                last_board = Some(boards.swap_remove(*indexes.last().unwrap()));
                for f in indexes.iter().take(indexes.len() - 1).rev() {
                    boards.swap_remove(*f);
                }
            }
        }
    }

    assert!(
        last_board.is_some() && last_winning_num.is_some(),
        "At least one board must win"
    );

    last_board.unwrap().sum_board_elem() * (last_winning_num.unwrap() as u64)
}

fn pre_processing(input: &mut Vec<String>) -> (Vec<u8>, Vec<Board>) {
    // Parse the numbers to draw
    let nums_to_draw: Vec<u8> = input
        .remove(0)
        .split(',')
        .map(|x| x.parse::<u8>().expect("Invalid number"))
        .collect();

    // Parse the boards
    let boards: Vec<Board> = input
        .chunks(6)
        .map(|raw_board| Board::new(&raw_board[1..]))
        .collect();

    (nums_to_draw, boards)
}

mod board {
    use std::fmt;

    pub struct Board {
        board: [[u8; 5]; 5],
        pub is_winner: bool,
    }

    impl Board {
        const FOUND_MARKER: u8 = u8::MAX;
        const WINNING_SUM: u16 = Board::FOUND_MARKER as u16 * 5;

        pub fn new(raw_board: &[String]) -> Board {
            assert_eq!(raw_board.len(), 5);
            let board = raw_board
                .iter()
                .map(|row| {
                    row.split_whitespace()
                        .map(|x| x.parse().expect("Failed to parse number"))
                        .collect::<Vec<u8>>()
                        .try_into()
                        .expect("Row length mismatch")
                })
                .collect::<Vec<[u8; 5]>>()
                .try_into()
                .expect("Board length mismatch");

            Board {
                board,
                is_winner: false,
            }
        }

        fn is_winner(&self) -> bool {
            let mut col_sum = [0u16; 5];

            for row in &self.board {
                let row_sum: u16 = row.iter().map(|&r| r as u16).sum();
                if row_sum == Board::WINNING_SUM {
                    return true;
                }

                for (i, &r) in row.iter().enumerate() {
                    col_sum[i] += r as u16;
                }
            }

            // Check if any column has the winning sum
            col_sum.iter().any(|&x| x == Board::WINNING_SUM)
        }

        pub fn mark_on_board(&mut self, num: u8) {
            for row in self.board.iter_mut() {
                if let Some(e) = row.iter_mut().find(|&&mut e| e == num) {
                    *e = Board::FOUND_MARKER;
                    break;
                }
            }
            self.is_winner = self.is_winner()
        }

        pub fn sum_board_elem(&self) -> u64 {
            assert!(self.is_winner, "Cannot sum up a board that's not a winner");

            self.board
                .iter()
                .flat_map(|row| row.iter())
                .filter(|&&e| e != Board::FOUND_MARKER)
                .map(|&e| e as u64)
                .sum()
        }
    }

    impl fmt::Debug for Board {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Write the is_winner field
            writeln!(f, "Board {{ is_winner: {} }}", self.is_winner)?;

            // Write the board field
            writeln!(f, "Board content:")?;
            for row in &self.board {
                for &elem in row {
                    write!(f, "{:3} ", elem)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
}
