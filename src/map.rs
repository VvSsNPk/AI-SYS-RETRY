use crate::map::square::Square;
mod square;
pub struct Map {
    pub map: Vec<Vec<Square>>,
    pub start: (usize, usize),
    pub portals: Vec<(usize, usize)>,
    pub moves : String,
}

impl Map {
    pub fn new() -> Self {
        Map {
            map: vec![vec![Default::default(); 18]; 12],
            start: (0, 0),
            portals: Vec::new(),
            moves: String::new(),
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
                                print!("   ")
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


    pub fn make_move(&self, c: char){
        if !&self.moves.is_empty(){
            let (first,second) = &self.start;
            if c.eq(&'N') || c.eq(&'S'){
               let mut count = 0;
               if c == 'N' {
                   count = 1;
               }else{
                   count = -1;
               }
               if first != &0usize && first != &11usize{
                   let move = first + count;
                   &mut self.start = (move,second);
                   &self.map[first][second];
               }
                
            } 


        }

        }
}
