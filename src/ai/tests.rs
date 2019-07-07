use super::*;
use super::Cell;
#[test]
fn make_move_test() {
    let cells = vec![Cell::Empty, Cell::Empty, Cell::Empty];

    let next_cell = Ai::make_move(&cells);
    match next_cell {
        Ok(val) => println!("{}", val),
        Err(why) => panic!("{}", why),  
    };
    assert!(true);
}

// if all cells are not empty
#[test]
fn make_move_test1() {
    let cells = vec![Cell::Cross, Cell::Empty, Cell::Cross];

    let next_cell = Ai::make_move(&cells);
    match next_cell {
        Ok(val) => println!("{}", val),
        Err(why) => panic!("{}", why),  
    };
    assert!(true);
}