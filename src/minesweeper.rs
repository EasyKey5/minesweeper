use std::{
    collections::HashSet,
    fmt::{Write,Display}
};
use crate::random::random_range;

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
                     if self.flagged_fields.contains(&pos) {
                       f.write_str("⚑ ")?;
                     } else {
                       f.write_str("🟪 ")?;
                     }
               } else if self.open_fields.contains(&pos) {
                   if self.mines.contains(&pos) {
                       f.write_str("💣 ")?;
                   } else{
                       write!(f, "{} ", self.neighbouring_mines(pos))?;
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

    fn open(&mut self, pos: Position) -> Option<OpenResult> {
        if self.flagged_fields.contains(&pos) {
           return None; 
        }
        self.open_fields.insert(pos);
        let is_mine = self.mines.contains(&pos);
        if is_mine {
            Some(OpenResult::Mine)
        } else {
            Some(OpenResult::NoMine(0))
        }
    }
    
    fn toggle_flag(&mut self, pos: Position) {
       if self.open_fields.contains(&pos) {
           return; 
        }

        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::{Minesweeper, random::random_range};

    #[test]
    fn test_ms() {
        let mut ms = Minesweeper::new(10, 10, 10);
        
        for i in 0..10 {
            ms.open((random_range(0, 10), random_range(0, 10)));
            if i % 2 == 0 {
                ms.toggle_flag((random_range(0, 10), random_range(0, 10)));
            }
        }
        println!("{}", ms);
    }
}
