use AI_SYS_RETRY::create_map;
fn main() {
    let x = create_map("problem_b_00.txt").unwrap();
    x.print_map();
    println!("{:?}", &x.portals);
    print!("{}",&x.moves);
}
