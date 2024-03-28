fn trapezio(a: f64, b: f64, n: usize, f: fn(f64) -> f64) -> f64 {
    let dx = (b - a) / (n as f64);
    ((0..=n)
        .map(|i| {
            let xi = a + dx * (i as f64);
            if i == 0 || i == n {
                f(xi)
            } else {
                2.0 * f(xi)
            }
        })
        .sum::<f64>()
        / 2.0)
        * dx
}

fn main() {
    let f = |x: f64| x;
    let a = 0.0;
    let b = 3.0;
    let n = 1000;
    println!("A integral é {}", trapezio(a, b, n, f));
}
