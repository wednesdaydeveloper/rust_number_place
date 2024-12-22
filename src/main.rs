// src/main.rs
fn main() {
    use number_place::sudoku::Sudoku;
    // 初期状態の盤面
    let array = [
            [0,0,0,0,0,0,0,0,0],
            [0,1,6,0,0,0,5,9,0],
            [4,0,0,3,0,7,0,0,2],
            [7,0,0,0,5,0,0,0,1],
            [8,0,0,0,0,0,0,0,9],
            [0,6,0,0,0,0,0,7,0],
            [0,0,5,0,0,0,6,0,0],
            [0,0,0,2,0,3,0,0,0],
            [0,0,0,0,7,0,0,0,0],
    ];

    let sudoku = Sudoku::new();
    if let Some(answers) = sudoku.solve_from_array(&array) {
        println!("Solved!.");
        sudoku.print(&answers)
    }
    else {
        println!("No solution found.");
    }
}