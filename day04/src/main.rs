use std::io::{self, Read};

/// solution is not very clean, I wasn't motivated

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let mut lines = buf.lines();

    let number_draws: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    lines.next();

    let mut boards: Vec<Vec<Vec<u32>>> = vec![];
    for (i, line) in lines.enumerate() {
        let i = i % 6;
        if i == 5 {
            debug_assert_eq!(boards.last().unwrap().len(), 5);
            continue; // empty line
        }
        if i == 0 {
            boards.push(vec![]);
        }
        boards
            .last_mut()
            .unwrap()
            .push(line.split(" ").filter_map(|s| s.parse().ok()).collect());

        debug_assert_eq!(boards.last().unwrap().last().unwrap().len(), 5);
    }
    let boards = boards;

    let mut marked: Vec<Vec<Vec<bool>>> = vec![];
    for _ in boards.iter() {
        marked.push(vec![vec![false; 5]; 5]);
    }

    fn has_complete_line(mark: &Vec<Vec<bool>>) -> bool {
        fn all_true(v: &Vec<bool>) -> bool {
            v.iter().fold(true, |a, x| a && *x)
        }
        mark.iter().fold(false, |a, x| a || all_true(x))
    }

    fn has_complete_col(mark: &Vec<Vec<bool>>) -> bool {
        let mut mark_transposed = vec![];
        for i in 0..5 {
            let mut col = vec![];
            for j in 0..5 {
                col.push(mark[j][i]);
            }
            mark_transposed.push(col);
        }

        has_complete_line(&mark_transposed)
    }

    fn unmarked_numbers(board: &Vec<Vec<u32>>, mark: &Vec<Vec<bool>>) -> Vec<u32> {
        let mut unmarked_numbers = vec![];
        for i in 0..5 {
            for j in 0..5 {
                if !mark[i][j] {
                    unmarked_numbers.push(board[i][j]);
                }
            }
        }
        unmarked_numbers
    }

    for n in number_draws {
        // mark numbers
        for (board, mark) in boards.iter().zip(marked.iter_mut()) {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == n {
                        mark[i][j] = true;
                    }
                }
            }
        }
        // check if a board has won
        for (board_index, mark) in marked.iter().enumerate() {
            if has_complete_line(mark) || has_complete_col(mark) {
                let score = unmarked_numbers(&boards[board_index], mark)
                    .iter()
                    .sum::<u32>()
                    * n;
                println!("{}", score); // prints the solution
                return Ok(());
            }
        }
    }

    panic!("No board wins");
}
