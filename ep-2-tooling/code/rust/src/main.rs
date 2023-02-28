use std::env;

fn read_first_arg() {
    let input_str = env::args().nth(1).unwrap_or("No input given".to_string());
    println!("The first argument provided is: {}.", input_str);
}

fn main() {
    read_first_arg();
}
