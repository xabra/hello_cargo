fn main() {
    let x = 3;
    let y = x;

    println!("x, y {}  {}", x, y);

    for n in 0..=10 {
        println!("n={}, {}", n, n * n);
    }

    let mut n = 0;
    while n < 100 {
        // Do stuff
        n += 1;
        println!("n={}", n);
    }
}
