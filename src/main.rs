use puzzleday::{Board, Block};
use std::cell::RefCell;

fn solve_puzzle<F>(board: &mut Board, blocks: &Vec<RefCell<Option<Block>>>,
                cb: &mut F)
where F: FnMut(&Board)
{
    let start_i: usize;
    let start_j: usize;

    //1. find first vacant cell on the board 
    match board.first_vacant() {
        None => {
            // no more vacant cell, this is a valid solution
            cb(board);
            return;
        }
        Some((i,j)) => {
            start_i = i;
            start_j = j;
        },
    }

    //2. apply available blocks one by one
    //for b in blocks.iter_mut().filter(|x| {if let Some(_) = x {true} else {false}}) {
    for b in blocks.iter() {
        let try_block = b.take();
        if let Some(mut block) = try_block {
            // There's a block, let's try fit it
            for f in &[false, true] {
                for r in &[puzzleday::Orientation::R0,puzzleday::Orientation::R90,
                puzzleday::Orientation::R180,puzzleday::Orientation::R270] {
                    block.flip(*f);
                    block.rotate(r);
                    'next: for j_offset in 0..block.cols() {
                        for cell in &block {
                            if start_j+cell.1 < j_offset || board.get_cell(start_i + cell.0, start_j+cell.1-j_offset) != '.' {
                                continue 'next;
                            }
                        }
                        if start_j >= j_offset {
                            // if no conflict, use this block and try recursively
                            board.apply_block(&block, start_i, start_j-j_offset);
                            solve_puzzle(board, blocks, cb);
                            // revert block from board after trying, move to the next orientation
                            board.revert_block(&block, start_i, start_j-j_offset);
                        }
                    }
                }
            }

            // Tried all positions, put this block back so it can be reused
            b.replace(Some(block));
        }

    }

}

fn main() {
    let mut board = Board::new(7,3);

    println!("Today's board is {}", board);

    let b1 = RefCell::new(Some(Block::new('1',4,2,vec![(0,0),(1,0),(2,0),(2,1),(3,1)])));
    let b2 = RefCell::new(Some(Block::new('2',4,2,vec![(0,0),(0,1),(1,1),(2,1),(3,1)])));
    let b3 = RefCell::new(Some(Block::new('3',4,2,vec![(0,0),(1,0),(2,0),(3,0),(1,1)])));
    let b4 = RefCell::new(Some(Block::new('4',3,3,vec![(0,2),(1,0),(1,1),(1,2),(2,0)])));
    let b5 = RefCell::new(Some(Block::new('5',3,3,vec![(0,0),(0,1),(0,2),(1,0),(2,0)])));
    let b6 = RefCell::new(Some(Block::new('6',3,2,vec![(0,0),(1,0),(2,0),(0,1),(1,1),(2,1)])));
    let b7 = RefCell::new(Some(Block::new('7',3,2,vec![(0,0),(1,0),(2,0),(0,1),(1,1)])));
    let b8 = RefCell::new(Some(Block::new('8',3,2,vec![(0,0),(1,0),(2,0),(0,1),(2,1)])));

    let blocks = vec![b1,b2,b3,b4,b5,b6,b7,b8];

    println!("Available blocks");
    for b in &blocks {
        if let Some(bb) = &*b.borrow() {
            println!("{}", bb);
        }
    }

    let mut total_solution = 0;
    solve_puzzle(&mut board, &blocks, &mut |solution|{
        total_solution+=1;
        println!("Solution:{}",solution);
    });
    println!("number of solutions:{}",total_solution);
}
