use std::env;

fn print_nth_arg(n: usize) {
    // Rust explicit try/except implementation:
    let input_str = match env::args().nth(n) {
        Some(arg) => arg,
        None => format!("No arg at position {n}."),
    };

    // Rust if/then implementation:
    let input_str = if env::args().count() > n {
        env::args().nth(n).unwrap()
    } else {
        format!("No arg at position {n}.")
    };

    // Rust idiomatic implementation:
    let input_str = env::args()
        .nth(n)
        .unwrap_or(format!("No arg at position {n}."));

    println!("The requested arg is: {input_str}.");
}

fn main() {
    print_nth_arg(1);
}
