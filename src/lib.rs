use crate::map::Map;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::fs::File;

mod map;

pub fn create_map(path :&str) -> Result<Map,Error>{
    let mut map = Map::new();
    let f =File::open(path)?;
    let  buffer = BufReader::new(f);
    let mut count = 0;
    let mut m = 0;
    for line in buffer.lines(){

        count = count +1;
        let store = match line {
            Err(_e) => panic!("no line"),
            Ok(line2) => line2,
        };
        if count < 3{
            continue;
        };
        let mut n = 0;

        for i in store.chars(){
            let x = &mut map.map[m][n];
            x.location = (m,n);
            if i == 'X'{
                x.is_wall = true;
            }else if i == 'P'{
                x.is_portal = true;
            }else if i == 'S'{
                x.cursor = true;

            }
            n = n + 1;
        }
        m = m + 1;

    }


    Ok(map)
}