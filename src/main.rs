use std::fs::File;
use std::io::{BufWriter, Write};
use AI_SYS_RETRY::{create_map, create_write_to_file};

fn main() {
    let x = create_map("problem_a_17.txt").unwrap();
    x.print_map();
}
