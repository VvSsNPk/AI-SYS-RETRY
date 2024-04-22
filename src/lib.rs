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
        let mut n = 0;
        count = count +1;
        let store = match line {
            Err(_e) => panic!("no line"),
            Ok(line2) => line2,
        };
        if count < 3{
            continue
        }
        let catcher = store.split("");

        for i in catcher{
            if i.eq("X"){
                map.map[m][n].is_wall = true;
            }else if i.eq("P"){
                map.map[m][n].is_portal = true;
            }else if i.eq("S"){
                map.map[m][n].cursor = true;
            }
            n = n + 1;
        }
        m = m + 1;

    }


    Ok(map)
}