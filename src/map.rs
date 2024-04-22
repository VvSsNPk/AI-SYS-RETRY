
use crate::map::square::Square;
mod square;

pub struct Map{
    pub map : Vec<Vec<Square>>,
}

impl Map{
    pub fn new() -> Self{
        Map{
            map: vec![vec![Default::default();18];12],
        }
    }

    pub fn print_map(&self){
        for i in &self.map{
            for j in i {
                if j.is_wall{
                    print!("\u{2588}\u{2588}");
                }else{
                    if j.cursor{
                        print!("\u{EE1D}");
                    } else{
                        if j.is_portal{
                            print!("\u{25A0}\u{25A0}");
                        }else{
                            if j.is_cleaned{
                                print!("  ")
                            }else{
                                print!("\u{25CC}\u{25CC}");
                            }
                        }
                    }
                }
                print!(" ")
            }
            println!()
        }
    }
}