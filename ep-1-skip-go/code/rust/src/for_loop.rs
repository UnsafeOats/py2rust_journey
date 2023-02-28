// Using for-loop with mutable list

pub fn implementation() {
    let a = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut b = Vec::new();

    for n in a {
        let new_val = n * 2;
        if new_val % 3 == 0 {
            b.push(new_val);
        };
    }

    println!("For Loop: {:?}", b);
}
