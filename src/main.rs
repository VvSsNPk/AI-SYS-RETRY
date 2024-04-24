use AI_SYS_RETRY::create_map;

fn main() {
    let x = create_map("problem_a_04.txt").unwrap();
    x.print_map();
    println!("{:?}", &x.portals);
    println!("{}", &x.moves);
    println!("{:?}", &x.uncleaned)
}
