#[derive(Debug, Clone, Copy)]
pub struct Sudoku {
    pub board: [[u8; 9]; 9],
}

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
}