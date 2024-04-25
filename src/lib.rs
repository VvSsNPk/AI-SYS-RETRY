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
                }else {
                    map.uncleaned.insert((m,n));
                }
                n = n + 1;
            }
            m = m + 1;
        }
    }
    println!("{:?}", moves);

    map.moves.push_str(&moves[1]);
    Ok(map)
}

pub fn simulate_moves(map: &mut Map){

    let str = map.moves.clone();
    for i in str.chars() {
        map.make_move(i);
    }
}

pub  fn create_write_to_file(path : &str) -> std::io::Result<()>{
    let mut sol = path.to_string();
    let mut result = create_map(path).unwrap();
    sol = sol.replace("problem", "solution");
    let mut f = File::create(sol)?;
    simulate_moves(&mut result);
    if !result.uncleaned.is_empty(){
        f.write_all("BAD PLAN".as_bytes())?;
        f.write_all(b"\n")?;
       for x in result.uncleaned{
           let (i,j) = x;
           f.write_all(i.to_string().as_bytes())?;
           f.write_all(b",")?;
           f.write_all(j.to_string().as_bytes())?;
           f.write_all(b"\n")?;
       }
    }else{
        f.write_all("GOOD PLAN".as_bytes())?;
    }

    
    Ok(())
}

