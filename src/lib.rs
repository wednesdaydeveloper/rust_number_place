pub mod sudoku{
    
    #[derive(Debug, Clone, Copy)]
    pub struct Sudoku {
        pub board: [[u8; 9]; 9],
    }

    #[allow(dead_code)]
    impl Sudoku {
        pub fn new(board: [[u8; 9]; 9]) -> Self {
            Sudoku { board }
        }
    
        fn is_valid(&self, row: usize, col: usize, num: u8) -> bool {
            // 行と列の重複チェック
            for i in 0..9 {
                if self.board[row][i] == num || self.board[i][col] == num {
                    return false;
                }
            }
    
            // ブロック内の重複チェック
            let box_row = row / 3;
            let box_col = col / 3;
            for i in 0..3 {
                for j in 0..3 {
                    if self.board[box_row * 3 + i][box_col * 3 + j] == num {
                        return false;
                    }
                }
            }
    
            true
        }
    
        pub fn solve(&mut self) -> bool {
            for row in 0..9 {
                for col in 0..9 {
                    if self.board[row][col] == 0 {
                        for num in 1..=9 {
                            if self.is_valid(row, col, num) {
                                self.board[row][col] = num;
                                if self.solve() {
                                    return true;
                                }
                                self.board[row][col] = 0;
                            }
                        }
                        return false;
                    }
                }
            }
            true
        }
        pub fn print(&self) {
            for row in &self.board {
                for num in row {
                    print!("{:2} ", num);
                }
                println!();
            }
        }
    
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hard_puzzle() {
        // 難しいパズルの初期データを作成
        let mut sudoku = sudoku::Sudoku::new([
            [0,0,0,0,0,0,0,0,0],
            [0,1,6,0,0,0,5,9,0],
            [4,0,0,3,0,7,0,0,2],
            [7,0,0,0,5,0,0,0,1],
            [8,0,0,0,0,0,0,0,9],
            [0,6,0,0,0,0,0,7,0],
            [0,0,5,0,0,0,6,0,0],
            [0,0,0,2,0,3,0,0,0],
            [0,0,0,0,7,0,0,0,0],
        ]);

        assert!(sudoku.solve());
        // 正しい解になっているか確認する
        assert_eq!(sudoku.board, [
            [2,8,7,5,1,9,4,3,6],
            [3,1,6,4,2,8,5,9,7],
            [4,5,9,3,6,7,1,8,2],
            [7,9,4,8,5,2,3,6,1],
            [8,3,1,7,4,6,2,5,9],
            [5,6,2,9,3,1,8,7,4],
            [9,7,5,1,8,4,6,2,3],
            [6,4,8,2,9,3,7,1,5],
            [1,2,3,6,7,5,9,4,8],
        ]);
    }

    #[test]
    fn test_no_solution() {
        // 解が存在しないパズルの初期データを作成
        let mut sudoku = sudoku::Sudoku::new([
            [0,0,0,0,0,0,0,0,0],
            [0,1,6,0,0,0,5,9,0],
            [4,0,0,8,0,7,0,0,2],
            [7,0,0,0,5,0,0,0,1],
            [8,0,0,0,0,0,0,0,9],
            [0,6,0,0,0,0,0,7,0],
            [0,0,5,0,0,0,6,0,0],
            [0,0,0,2,0,3,0,0,0],
            [0,0,0,0,7,0,0,0,0],
        ]);

        assert!(!sudoku.solve());
    }

    #[test]
    fn test_boundary_case_many_solution() {
        // 解が複数ある場合
        let mut sudoku = sudoku::Sudoku::new([
            [0,0,0,0,0,0,0,0,0],
            [0,1,6,0,0,0,5,9,0],
            [4,0,0,1,0,7,0,0,2],
            [7,0,0,0,5,0,0,0,1],
            [8,0,0,0,0,0,0,0,9],
            [0,6,0,0,0,0,0,7,0],
            [0,0,5,0,0,0,6,0,0],
            [0,0,0,2,0,3,0,0,0],
            [0,0,0,0,7,0,0,0,0],
        ]);

        assert!(!sudoku.solve());
    }

    // 他の境界値テストを追加
}