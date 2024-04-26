use std::collections::HashSet;
use crate::map::square::Square;
mod square;
#[derive(Clone)]
pub struct Map {
    pub map: [[Square;18];12],
    pub start: (usize, usize),
    pub portals: Vec<(usize,usize)>,
    pub moves : String,
    pub uncleaned : HashSet<(usize,usize)>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            map: [[Default::default(); 18]; 12],
            start: (0, 0),
            portals: Vec::new(),
            moves: String::new(),
            uncleaned: HashSet::new(),
        }
    }

    pub fn print_map(&self) {
        for i in &self.map {
            for j in i {
                if j.is_wall {
                    print!("X");
                } else {
                    if j.cursor {
                        print!("S");
                    } else {
                        if j.is_portal {
                            print!("P");
                        } else {
                            if j.is_cleaned {
                                print!(" ")
                            } else {
                                print!("#");
                            }
                        }
                    }
                }
                print!(" ")
            }
            println!()
        }
    }


    pub fn make_move(&mut self, c: char){
        if !&self.moves.is_empty(){
            let (first,second) = self.start;
            if c.eq(&'N') || c.eq(&'S'){
               if first != 0usize && first != 11usize{
                   let  ch;
                   if c == 'N'{
                    ch = first -  1usize;
                   } else{
                    ch = first + 1usize;
                   }
                   if !self.map[ch][second].is_wall {
                       self.map[first][second].is_cleaned = true;
                       self.map[first][second].cursor = false;
                       if !self.map[ch][second].is_portal {
                           self.start = (ch, second);
                           self.map[ch][second].is_cleaned = true;
                           self.map[ch][second].cursor = true;
                           self.uncleaned.remove(&self.start);
                       }else{
                           let mut m = 0usize;
                           let mut n=0usize;
                           for i in &self.portals{
                               if i != &(ch,second){
                                   (m,n) = *i;
                               }
                           }
                           self.start = (m,n);
                           self.map[m][n].cursor = true;
                       }
                   }
                }
                
            }else{
                if second != 0usize && second != 17usize{

                    let ch;
                    if c == 'W'{
                        ch = second - 1usize;
                    }else{
                        ch = second + 1usize;
                    }

                    if !self.map[first][ch].is_wall {
                        self.map[first][second].is_cleaned = true;
                        self.map[first][second].cursor = false;
                        if !self.map[first][ch].is_portal {
                            self.start = (first, ch);
                            self.map[first][ch].cursor = true;
                            self.map[first][ch].is_cleaned = true;
                            self.uncleaned.remove(&self.start);
                        }else{
                            let mut m = 0usize;
                            let mut n=0usize;
                            for i in &self.portals{
                                if i != &(ch,second){
                                    (m,n) = *i;
                                }
                            }
                            self.start = (m,n);
                            self.map[m][n].cursor = true;
                        }
                    }
                }
            }
        }
    }

}
