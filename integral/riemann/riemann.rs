fn riemann(a: f64, b: f64, n: usize, f: fn(f64) -> f64) -> f64 {
    let dx = (b - a) / (n as f64);
    (0..=n)
        .map(|i| {
            print!("{}/{}\r", i, n);
            dx * f(a + dx * (i as f64))
        })
        .sum::<f64>()
}

fn main() {
    let f = |x: f64| x;
    let a = 0.0;
    let b = 3.0;
    let n = 1000;
    println!("A integral é {}", riemann(a, b, n, f));
}
