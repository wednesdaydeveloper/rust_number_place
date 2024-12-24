// src/main.rs
fn main() {
    use number_place::sudoku::solve_from_array;
    use number_place::sudoku::print;
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

    if let Some(answers) = solve_from_array(&array) {
        println!("Solved!.");
        print(&answers)
    }
    else {
        println!("No solution found.");
    }
}