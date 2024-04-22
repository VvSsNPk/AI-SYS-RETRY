
use crate::map::square::Square;
mod square;

pub struct Map{
    pub map : Vec<Vec<Square>>,
}

impl Map{
    pub fn new() -> Self{
        Map{
            map: vec![vec![Default::default();17];11],
        }
    }

    pub fn print_map(&self){
        for i in &self.map{
            for j in i {
                if j.is_wall{
                    print!("X");
                }else{
                    if j.cursor{
                        print!("S")
                    } else{
                        if j.is_cleaned{
                            print!("   ")
                        }else{
                            print!("#")
                        }
                    }
                }
                print!(" ")
            }
            println!()
        }
    }
}