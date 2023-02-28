pub fn implementation() {
    let out = cute::c![n * 2, for n in 0..10, if n % 3 == 0];

    println!("List Comprehension Output: {:?}", out);
}
