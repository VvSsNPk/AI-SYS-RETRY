use crate::map::Map;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

mod map;

pub fn create_map(path: &str) -> Result<Map, Error> {
    let mut map = Map::new();
    let f = File::open(path)?;
    let mut buffer = BufReader::new(f);
    let mut m = 0;
    let mut moves = Vec::new();
    for line in buffer.lines() {
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
    if result.start == (0,0){
        start_not_given(&mut result, &mut f).expect("Error!");
    }else {
        simulate_moves(&mut result);
        write_to_file(&result, &mut f).expect("Error writing to file");
    }

    
    Ok(())
}

pub fn write_to_file(map: &Map, f: &mut File) -> std::io::Result<()> {
    if !&map.uncleaned.is_empty(){
        f.write_all("BAD PLAN".as_bytes())?;
        f.write_all(b"\n")?;
        for x in &map.uncleaned{
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

pub fn start_not_given(map: &mut Map, f: &mut File) -> std::io::Result<()> {
    let uncleaned  = &map.uncleaned.clone();
    for i in uncleaned{
            let (m, n) = i.clone();
            map.start = (m,n);
            map.map[m][n].cursor = true;
            simulate_moves(map);
            write_to_file(&map,f).expect("Couldn't write to file");
            map.map[m][n].cursor = false;
        }
    
    
    Ok(())
}

