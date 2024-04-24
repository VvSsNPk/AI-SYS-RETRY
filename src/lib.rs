use crate::map::Map;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

mod map;

pub fn create_map(path: &str) -> Result<Map, Error> {
    let mut map = Map::new();
    let f = File::open(path)?;
    let mut buffer = BufReader::new(f);
    let mut count = 0;
    let mut m = 0;
    let mut moves = Vec::new();
    for line in buffer.lines() {
        count += 1;
        let store = match line {
            Err(_e) => panic!("no line"),
            Ok(line2) => line2,
        };
        if !store.contains("X"){
            moves.push(store);
            continue;
        }
        else{
            let mut n = 0;
            for i in store.chars() {
                let x = &mut map.map[m][n];
                x.location = (m, n);
                if i == 'X' {
                    x.is_wall = true;
                } else if i == 'P' {
                    x.is_portal = true;
                    map.portals.push((m, n));
                } else if i == 'S' {
                    x.cursor = true;
                    map.start = (m, n);
                }
                n = n + 1;
            }
            m = m + 1;
        }
    }
    println!("{:?}", moves);

    map.moves.push_str(&moves[1]);
   let str = &map.moves.clone();
    for i in str.chars() {
        &map.make_move(i);
    }


    Ok(map)
}

