pub mod sudoku {

    #[derive(Debug, Clone, Copy)]
    pub struct Cell {
        pub row: u8,
        pub col: u8,
    }

    impl Cell {
        pub fn block(&self) -> u8 {
            (self.row / 3) * 3 + self.col / 3
        }

        pub fn unused(&self, solved: &Vec<SolvedCell>) -> Vec<u8> {
            let solved_numbers = solved
                .iter()
                .filter(|&solved_cell| {
                    solved_cell.cell.row == self.row
                        || solved_cell.cell.col == self.col
                        || solved_cell.cell.block() == self.block()
                })
                .map(|&solved_cell| solved_cell.val)
                .collect::<Vec<u8>>();

            return (1..=9).filter(|&n| !solved_numbers.contains(&n)).collect();
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct SolvedCell {
        pub cell: Cell,
        pub val: u8,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Sudoku {}

    impl Sudoku {
        // インスタンス生成する関連関数
        pub fn new() -> Sudoku {
            Sudoku {}
        }

        pub fn print(&self, solved: &Vec<SolvedCell>) {
            for row in 0..9 {
                for col in 0..9 {
                    let n = match solved.iter().find(|&solved_cell| {
                        solved_cell.cell.row == row && solved_cell.cell.col == col
                    }) {
                        Some(solved_cell) => solved_cell.val,
                        None => 0,
                    };
                    print!("{:2} ", n);
                }
                println!();
            }
        }
        pub fn solve_from_array(&self, array: &[[u8; 9]; 9]) -> Option<Vec<SolvedCell>> {
            let mut solved: Vec<SolvedCell> = Vec::new();
            for row in 0..9 {
                for col in 0..9 {
                    if array[row][col] != 0 {
                        solved.push(SolvedCell {
                            cell: Cell {
                                row: row as u8,
                                col: col as u8,
                            },
                            val: array[row][col],
                        });
                    }
                }
            }
            self.print(&solved);
            self.solve(&solved)
        }

        fn solve(&self, solved: &Vec<SolvedCell>) -> Option<Vec<SolvedCell>> {
            let mut unsolved: Vec<Cell> = Vec::new();
            for row in 0..9 {
                for col in 0..9 {
                    if !solved.iter().any(|&solved_cell| {
                        solved_cell.cell.row == row && solved_cell.cell.col == col
                    }) {
                        unsolved.push(Cell { row, col });
                    }
                }
            }

            if unsolved.is_empty() {
                return Some(solved.clone());
            } else {
                if unsolved.iter().all(|&cell| cell.unused(&solved).len() > 0) {
                    if let Some(cell) = unsolved
                        .iter()
                        .min_by_key(|&cell| cell.unused(&solved).len())
                    {
                        let answers: Vec<Vec<SolvedCell>> = cell
                            .unused(&solved)
                            .iter()
                            .filter_map(|&val| {
                                self.solve(
                                    &[solved.clone(), vec![SolvedCell { cell: *cell, val }]]
                                        .concat(),
                                )
                            })
                            .collect::<Vec<Vec<SolvedCell>>>();
                        if answers.len() == 1 {
                            return Some((&answers[0]).clone());
                        }
                    }
                }
            }
            None::<Vec<SolvedCell>>
        }

        pub fn answer_to_array(&self, answer: &Vec<SolvedCell>) -> [[u8; 9]; 9] {
            let mut array: [[u8; 9]; 9] = [[0; 9]; 9];
            for row in 0..9 {
                for col in 0..9 {
                    if let Some(n) = answer.iter().find(|&solved_cell| {
                        solved_cell.cell.row == row && solved_cell.cell.col == col
                    }) {
                        array[row as usize][col as usize] = n.val;
                    }
                }
            }
            return array;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::Sudoku;
    #[test]
    fn test_hard_puzzle() {
        // 難しいパズルの初期データを作成
        let sudoku = Sudoku::new();
        let answers = sudoku.solve_from_array(&[
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 6, 0, 0, 0, 5, 9, 0],
            [4, 0, 0, 3, 0, 7, 0, 0, 2],
            [7, 0, 0, 0, 5, 0, 0, 0, 1],
            [8, 0, 0, 0, 0, 0, 0, 0, 9],
            [0, 6, 0, 0, 0, 0, 0, 7, 0],
            [0, 0, 5, 0, 0, 0, 6, 0, 0],
            [0, 0, 0, 2, 0, 3, 0, 0, 0],
            [0, 0, 0, 0, 7, 0, 0, 0, 0],
        ]);
        assert!(answers.is_some());
        // 正しい解になっているか確認する
        assert_eq!(
            sudoku.answer_to_array(&answers.unwrap()),
            [
                [2, 8, 7, 5, 1, 9, 4, 3, 6],
                [3, 1, 6, 4, 2, 8, 5, 9, 7],
                [4, 5, 9, 3, 6, 7, 1, 8, 2],
                [7, 9, 4, 8, 5, 2, 3, 6, 1],
                [8, 3, 1, 7, 4, 6, 2, 5, 9],
                [5, 6, 2, 9, 3, 1, 8, 7, 4],
                [9, 7, 5, 1, 8, 4, 6, 2, 3],
                [6, 4, 8, 2, 9, 3, 7, 1, 5],
                [1, 2, 3, 6, 7, 5, 9, 4, 8],
            ]
        );
    }

    #[test]
    fn test_no_solution() {
        // 解が存在しないパズルの初期データを作成
        let sudoku = Sudoku::new();
        let answers = sudoku.solve_from_array(&[
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 6, 0, 0, 0, 5, 9, 0],
            [4, 0, 0, 8, 0, 7, 0, 0, 2],
            [7, 0, 0, 0, 5, 0, 0, 0, 1],
            [8, 0, 0, 0, 0, 0, 0, 0, 9],
            [0, 6, 0, 0, 0, 0, 0, 7, 0],
            [0, 0, 5, 0, 0, 0, 6, 0, 0],
            [0, 0, 0, 2, 0, 3, 0, 0, 0],
            [0, 0, 0, 0, 7, 0, 0, 0, 0],
        ]);

        assert!(answers.is_none());
    }

    #[test]
    fn test_boundary_case_many_solution() {
        // 解が複数ある場合
        let sudoku = Sudoku::new();
        let answers = sudoku.solve_from_array(&[
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 6, 0, 0, 0, 5, 9, 0],
            [4, 0, 0, 1, 0, 7, 0, 0, 2],
            [7, 0, 0, 0, 5, 0, 0, 0, 1],
            [8, 0, 0, 0, 0, 0, 0, 0, 9],
            [0, 6, 0, 0, 0, 0, 0, 7, 0],
            [0, 0, 5, 0, 0, 0, 6, 0, 0],
            [0, 0, 0, 2, 0, 3, 0, 0, 0],
            [0, 0, 0, 0, 7, 0, 0, 0, 0],
        ]);

        assert!(answers.is_none());
    }

    // 他の境界値テストを追加
}
