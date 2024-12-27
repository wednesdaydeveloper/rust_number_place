pub mod sudoku {
    use itertools::{iproduct, Itertools};

    #[derive(Debug, Clone, Copy)]
    pub struct Cell {
        pub row: u8,
        pub col: u8,
    }

    impl Cell {
        pub fn block(&self) -> u8 {
            (self.row / 3) * 3 + self.col / 3
        }

        pub fn unused(&self, solved: &SolvedCells) -> Vec<u8> {
            let solved_numbers = solved
                .iter()
                .filter(|&solved_cell| {
                    solved_cell.cell.row == self.row
                        || solved_cell.cell.col == self.col
                        || solved_cell.cell.block() == self.block()
                })
                .map(|&solved_cell| solved_cell.val)
                .collect::<Vec<u8>>();

            (1..=9).filter(|&n| !solved_numbers.contains(&n)).collect()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct SolvedCell {
        pub cell: Cell,
        pub val: u8,
    }
    
    pub type SolvedCells = Vec<SolvedCell>;

    fn to_row_col() -> itertools::Product<std::ops::Range<u8>, std::ops::Range<u8>> {
        iproduct!(0..9, 0..9)
    }

    pub fn to_string(solved: &SolvedCells) -> String {
        to_row_col()
            .map(|(row, col)| {
                format!(
                    "{:2}",
                    match solved
                        .iter()
                        .find(|&solved_cell| solved_cell.cell.row == row
                            && solved_cell.cell.col == col)
                    {
                        Some(solved_cell) => solved_cell.val,
                        None => 0,
                    }
                )
            })
            .chunks(9) //  1行ごとにまとめる
            .into_iter()
            .map(|mut x| x.join(" "))
            .join("\r\n")
    }

    pub fn solve_from_array(array: &[[u8; 9]; 9]) -> Option<SolvedCells> {
        let solved = to_row_col()
            .filter(|&(row, col)| array[row as usize][col as usize] != 0)
            .map(|(row, col)| SolvedCell {
                cell: Cell { row, col },
                val: array[row as usize][col as usize],
            })
            .collect();

        println!("{}", to_string(&solved));
        solve(&solved)
    }

    fn solve(solved: &SolvedCells) -> Option<SolvedCells> {
        //  未入力のセルを抽出
        let unsolved = to_row_col()
            .filter(|&(row, col)| {
                !solved
                    .iter()
                    .any(|&solved_cell| solved_cell.cell.row == row && solved_cell.cell.col == col)
            })
            .map(|(row, col)| Cell { row, col })
            .collect::<Vec<Cell>>();

        if unsolved.is_empty() {
            //  未入力のセルがない？
            return Some(solved.clone());
        } else {
            //  未入力のセルで入力できる数値のリスト（候補リスト）を生成し、そのリストが最も短いものを求める
            if let Some((unused, cell)) = unsolved
                .iter()
                //  候補リストとセルのタプルを作成
                .map(|&cell| (cell.unused(solved), cell))
                //  候補リストの長さが最小のものを取得
                .min_by_key(|(unused, _)| unused.len())
            {
                //  その候補リストの値を1つセットして再帰的に solve を実行する。
                let answers: Vec<SolvedCells> = unused
                    .iter()
                    .filter_map(|&val| {
                        let new_solved = [solved.clone(), vec![SolvedCell { cell, val }]].concat();
                        solve(&new_solved)
                    })
                    .collect::<Vec<SolvedCells>>();
                if answers.len() == 1 {
                    //  解が1つだけの場合のみ、正解！
                    return Some(answers[0].clone());
                }
            }
        }
        None::<SolvedCells>
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::solve_from_array;
    use crate::sudoku::SolvedCells;

    pub fn answer_to_array(answer: &SolvedCells) -> [[u8; 9]; 9] {
        let mut array: [[u8; 9]; 9] = [[0; 9]; 9];
        for row in 0..9 {
            for col in 0..9 {
                if let Some(n) = answer
                    .iter()
                    .find(|&solved_cell| solved_cell.cell.row == row && solved_cell.cell.col == col)
                {
                    array[row as usize][col as usize] = n.val;
                }
            }
        }
        return array;
    }

    #[test]
    fn test_hard_puzzle() {
        // 難しいパズルの初期データを作成
        // let sudoku = Sudoku::new();
        let answers = solve_from_array(&[
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
            answer_to_array(&answers.unwrap()),
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
        let answers = solve_from_array(&[
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
        let answers = solve_from_array(&[
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
