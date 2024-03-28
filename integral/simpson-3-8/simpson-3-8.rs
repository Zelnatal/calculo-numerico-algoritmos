fn simpson(a: f64, b: f64, n: usize, f: fn(f64) -> f64) -> Result<f64, &'static str> {
    if !(n % 3 == 0 && n >= 3) {
        return Err("O n tem que ser divisível por 3 maior ou igual a 3");
    }
    let h = (b - a) / (n as f64);
    Ok((0..=n)
        .map(|i| {
            let xi = a + h * (i as f64);
            match i {
                _ if i == 0 || i == n => f(xi),
                _ if i % 3 == 1 || i % 3 == 2 => 3.0 * f(xi),
                _ if i % 3 == 0 => 2.0 * f(xi),
                _ => unreachable!(),
            }
        })
        .sum::<f64>()
        * (3.0 * h / 8.0))
}

fn main() {
    let f = |x: f64| x;
    let a = 0.0;
    let b = 3.0;
    let n = 1002;
    match simpson(a, b, n, f) {
        Ok(r) => println!("A integral é {}", r),
        Err(s) => eprintln!("Erro: {}", s),
    }
}
