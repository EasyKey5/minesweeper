#![allow(dead_code)]

mod random;
use std::{collections::HashSet, fmt::Write};
use random::random_range;
use std::fmt::Display;

type Position = (usize, usize);
enum OpenResult {
    Mine,
    NoMine(u8)
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,    
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>
}

impl Display for Minesweeper {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       for y in 0..self.height {
           for x in 0..self.width {
               let pos = (x,y);
               
               if !self.open_fields.contains(&pos) {
                       f.write_str("🟪")?;
               } else if self.open_fields.contains(&pos) {
                   if self.mines.contains(&pos) {
                       f.write_str("💣")?;
                   } else {
                       write!(f, "{}", self.neighbouring_mines(pos))?;
                   }
               }
            }
           f.write_char('\n')?;
        }
       Ok(())
   } 
}

impl Minesweeper {
    pub fn new(width:usize, height:usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                     mines.insert((random_range(0, width), random_range(0, height)));
                }
                mines
            },
            flagged_fields: HashSet::new()}
    }

    fn iter_neighbours(&self, (x,y): Position) -> impl Iterator<Item = Position> {

        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |x| (y.max(1) - 1..=(y + 1).min(height - 1))
                .map(move |y| (x,y)))
            .filter(move |&pos| pos != (x,y))
    }
    
    fn neighbouring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbours(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    fn open(&mut self, position: Position) -> OpenResult {
        self.open_fields.insert(position);
        let is_mine = self.mines.contains(&position);
        if is_mine {
            OpenResult::Mine
        } else {
            OpenResult::NoMine(0)
        }
    }
    

}


#[cfg(test)]
mod tests {
    use crate::{Minesweeper, random::random_range};

    #[test]
    fn test_ms() {
        let mut ms = Minesweeper::new(10, 10, 10);
        
        for _ in 0..10 {
            ms.open((random_range(0, 10), random_range(0, 10)));
        }
        println!("{}", ms);
    }
}
