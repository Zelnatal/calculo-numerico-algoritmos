fn quadratura_gaussiana(a: f64, b: f64, f: fn(f64) -> f64) -> f64 {
    let x = |t: f64| (b - a) * t / 2.0 + (b + a) / 2.0;
    (f(x(-(3.0_f64.sqrt() / 3.0))) + f(x(3.0_f64.sqrt() / 3.0))) * (b - a) / 2.0
}

fn main() {
    let f = |x: f64| x;
    let a = 0.0;
    let b = 3.0;
    println!("A integral é {}", quadratura_gaussiana(a, b, f));
}
