fn simpson(a: f64, b: f64, n: usize, f: fn(f64) -> f64) -> Result<f64, &'static str> {
    if !(n % 2 == 0 && n >= 2) {
        return Err("O n tem que ser par maior ou igual a 2");
    }
    let h = (b - a) / (n as f64);
    Ok((0..=n)
        .map(|i| {
            let xi = a + h * (i as f64);
            match i {
                _ if i == 0 || i == n => f(xi),
                _ if i % 2 == 1 => 4.0 * f(xi),
                _ if i % 2 == 0 => 2.0 * f(xi),
                _ => unreachable!(),
            }
        })
        .sum::<f64>()
        * (h / 3.0))
}

fn main() {
    let f = |x: f64| x;
    let a = 0.0;
    let b = 3.0;
    let n = 1000;
    match simpson(a, b, n, f) {
        Ok(r) => println!("A integral é {}", r),
        Err(s) => eprintln!("Erro: {}", s),
    }
}
