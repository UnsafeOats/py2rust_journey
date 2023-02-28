// Using list comprehensions

pub fn implementation() {
    let out = cute::c![n * 2, for n in 0..10, if (n * 2) % 3 == 0];
    println!("List Comp: {:?}", out);
}
