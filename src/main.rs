// src/main.rs
fn main() {
    use number_place::sudoku::Sudoku;
    // 初期状態の盤面
    let mut sudoku = Sudoku::new([
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
    if sudoku.solve() {
        println!("Solved:");
        sudoku.print();
    } else {
        println!("No solution found.");
    }
}