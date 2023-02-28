pub fn implementation() {
    let out: Vec<i32> = (0..10)
        .map(|n| n * 2)
        .filter(|n| n % 3 == 0)
        .collect();

    println!("Map & Filter Output: {:?}", out);
}
