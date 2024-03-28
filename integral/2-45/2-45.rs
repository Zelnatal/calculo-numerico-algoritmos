fn integral(a: f64, b: f64, n: usize, f: fn(f64) -> f64) -> Result<f64, &'static str> {
    if !(n % 4 == 0 && n >= 4) {
        return Err("O n tem que ser divisível por 4 maior ou igual a 3");
    }
    let h = (b - a) / (n as f64);
    Ok((0..=n)
        .map(|i| {
            let xi = a + h * (i as f64);
            match i {
                _ if i == 0 || i == n => 7.0 * f(xi),
                _ if i % 4 == 1 || i % 4 == 3 => 32.0 * f(xi),
                _ if i % 4 == 2 => 12.0 * f(xi),
                _ if i % 4 == 0 => 14.0 * f(xi),
                _ => unreachable!(),
            }
        })
        .sum::<f64>()
        * (2.0 * h / 45.0))
}

fn main() {
    let f = |x: f64| x;
    let a = 0.0;
    let b = 3.0;
    let n = 1004;
    match integral(a, b, n, f) {
        Ok(r) => println!("A integral é {}", r),
        Err(s) => eprintln!("Erro: {}", s),
    }
}
