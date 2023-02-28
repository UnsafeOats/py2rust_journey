// Using for-loop with mutable list

pub fn implementation() {
    let out: Vec<u16> = (0..10)
        .map(|n| n * 2)
        .filter(|n| n % 3 == 0)
        .collect();
    println!("Map & Filter: {:?}", out);
}
