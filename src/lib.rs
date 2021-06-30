#[derive(Debug, Clone, Copy)]
pub enum Orientation{
    R0,
    R90,
    R180,
    R270,
}

/// A struct representing a block piece
/// e.g.
///
/// x 0
/// x 0
/// x x
/// 0 x
///
/// Above is a 4x2 block
#[derive(Debug)]
pub struct Block {
    pub mark: char,
    num_rows: usize,
    num_cols: usize,
    orientation: Orientation,
    flip: bool,
    indices: Vec<(usize, usize)>,
}

/// Convient trait impl to do range for over the block pixels
/// Only the reference version &'a and &'a mut Block is provided
impl<'a> IntoIterator for &'a Block {
    type Item = (usize, usize);
    type IntoIter = BlockIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BlockIterator {
            block: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a mut Block {
    type Item = (usize, usize);
    type IntoIter = BlockIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BlockIterator {
            block: self,
            index: 0,
        }
    }
}

/// The Iterator you get when you call into_iter on a Block reference
pub struct BlockIterator<'a> {
    block: &'a Block,
    index: usize,
}

impl<'a> Iterator for BlockIterator<'a> {
    type Item = (usize,usize);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref xy) = self.block.indices.get(self.index) {
            self.index += 1;
            if self.block.flip {
                match self.block.orientation {
                    Orientation::R0 => Some((self.block.num_rows - xy.0 - 1, xy.1)),
                    Orientation::R90 => Some((xy.1, xy.0)),
                    Orientation::R180 => Some((xy.0, self.block.num_cols - xy.1 - 1)),
                    Orientation::R270 => Some((self.block.num_cols - xy.1 - 1, self.block.num_rows- xy.0 - 1)),
                }
            } else {
                match self.block.orientation {
                    Orientation::R0 => Some((xy.0, xy.1)),
                    Orientation::R90 => Some((self.block.num_cols - xy.1 - 1, xy.0)),
                    Orientation::R180 => Some((self.block.num_rows - xy.0 - 1, self.block.num_cols - xy.1 - 1)),
                    Orientation::R270 => Some((xy.1, self.block.num_rows- xy.0 - 1)),
                }
            }
        } else {
        None
        }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, ff: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut map = vec![' '; self.rows() * self.cols()];
        for xy in self {
            map[xy.0 * self.cols() + xy.1] = self.mark;
        }
        for i in 0..self.rows() {
            writeln!(ff).unwrap();
            for j in 0..self.cols() {
                write!(ff,"{}  ",map[i*self.cols() + j]).unwrap();
            }
        }
        Ok(())
    }
}

impl Block {
    pub fn new(mark: char, num_rows: usize, num_cols: usize, indices: Vec<(usize, usize)>) -> Block {
        for (i,j) in &indices {
            assert!(*i < num_rows);
            assert!(*j < num_cols);
        }
        Block {
            mark,
            num_rows,
            num_cols,
            orientation: Orientation::R0,
            flip: false,
            indices,
        }
    }

    pub fn rotate(&mut self, t: &Orientation) {
        self.orientation = *t;
    }

    pub fn flip(&mut self, f: bool) {
        self.flip = f;
    }

    pub fn rows(&self) -> usize {
        match self.orientation {
            Orientation::R0 | Orientation::R180 => self.num_rows,
            Orientation::R90 | Orientation::R270 => self.num_cols,
        }
    }

    pub fn cols(&self) -> usize {
        match self.orientation {
            Orientation::R0 | Orientation::R180 => self.num_cols,
            Orientation::R90 | Orientation::R270 => self.num_rows,
        }
    }
}

/// A struct to represent the puzzle board
pub struct Board {
    month: usize,
    day: usize,
    matrix: [[char; 7]; 7],
}

impl Board {
    pub fn new(month: usize, day: usize) -> Board {
        assert!(month <= 12);
        // needs better error handling on valid days for each month
        assert!(day <= 31);
        let mut b = Board {
            month,
            day,
            matrix: [['.'; 7]; 7],
        };
        b.matrix[0][6] = 'x';
        b.matrix[1][6] = 'x';
        b.matrix[6][3] = 'x';
        b.matrix[6][4] = 'x';
        b.matrix[6][5] = 'x';
        b.matrix[6][6] = 'x';

        b.matrix[(month - 1) / 6][(month - 1) % 6] = 'M';
        b.matrix[2 + (day - 1) / 7][(day - 1) % 7] = 'D';
        b
    }

    pub fn first_vacant(&self) -> Option<(usize, usize)> {
        for i in 0..7 {
            for j in 0..7 {
                if self.matrix[i][j] == '.' {
                    return Some((i,j));
                }
            }
        }
        None
    }

    pub fn get_cell(&self, i:usize, j:usize) -> char {
        if i > 6 || j > 6 {
           'x'
        } else {
           self.matrix[i][j]
        }
    }

    pub fn apply_block(&mut self, b: &Block, i:usize, j:usize) {
        for (ii,jj) in b {
            if ii + i < 7 && jj + j < 7 {
                self.matrix[ii+i][jj+j] = b.mark;
            }
        }
    }

    pub fn revert_block(&mut self, b: &Block, i:usize, j:usize) {
        for (ii,jj) in b {
            if ii + i < 7 && jj + j < 7 {
                self.matrix[ii+i][jj+j] = '.';
            }
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, ff: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..7 {
            writeln!(ff).unwrap();
            for j in 0..7 {
                write!(ff,"{}  ",self.matrix[i][j]).unwrap();
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: make proper test instead of visual inspection
    #[test]
    fn create_block() {
        let mut b1 = Block::new('G',4,2,vec![(0,0),(1,0),(2,0),(2,1),(3,1)]);
        println!("R0: {}", b1);
        b1.orientation = Orientation::R90;
        println!("R90: {}", b1);
        b1.orientation = Orientation::R180;
        println!("R180: {}", b1);
        b1.orientation = Orientation::R270;
        println!("R270: {}", b1);

        b1.flip = true;
        b1.orientation = Orientation::R0;
        println!("F-R0: {}", b1);
        b1.orientation = Orientation::R90;
        println!("F-R90: {}", b1);
        b1.orientation = Orientation::R180;
        println!("F-R180: {}", b1);
        b1.orientation = Orientation::R270;
        println!("F-R270: {}", b1);
    }

    #[test]
    fn create_board() {
        let bd = Board::new(6,28);
        println!("Board status: {}", bd);
    }
}
