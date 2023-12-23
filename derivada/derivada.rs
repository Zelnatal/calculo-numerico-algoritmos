type Função = fn(f64) -> f64;

const H: f64 = 1e-7;

fn derivada_1(f: &Função, x: f64) -> f64{
    (f(x+H)-f(x))/H
}

fn derivada_2(f: &Função, x: f64) -> f64{
    (f(x) - f(x-H))/H
}

fn derivada_3(f: &Função, x: f64) -> f64{
    (f(x+H) - f(x-H))/(2.0*H)
}

fn main() {
    let f: Função = |x| x.powf(3.0*x) + 4.0*x.powi(2) + x + 4.0 + (3.0*x.sqrt() + x.powi(99)).sqrt();
    let x = 0.3;

    println!("Método 1: {}",derivada_1(&f, x));
    println!("Método 2: {}",derivada_2(&f, x));
    println!("Método 3: {}",derivada_3(&f, x))
}