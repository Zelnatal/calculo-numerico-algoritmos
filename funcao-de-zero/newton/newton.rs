const H: f64 = 1e-7;
const MÉTODO: u32 = 2;

enum Erro {
    DNaN,
}

type Função = fn(f64) -> f64;

fn d(f: &Função, x: f64) -> f64 {
    match MÉTODO {
        0 => (f(x + H) - f(x)) / H,
        1 => (f(x) - f(x - H)) / H,
        2 => (f(x + H) - f(x - H)) / (2.0 * H),
        _ => panic!("Método de derivada não existe")
    }
}

fn newton(f: &Função, epsilon: f64, mut x: f64) -> Result<f64, Erro> {
    let mut fx = f(x);
    let mut dx = x - fx / d(f, x);
    let mut k = 0;

    if dx.is_nan() {
        return Err(Erro::DNaN);
    }

    while (x - dx).abs() >= epsilon && fx.abs() >= epsilon {
        x = dx;
        fx = f(x);
        dx = x - fx / d(f, x);
        if dx.is_nan() {
            return Err(Erro::DNaN);
        }

        k += 1;
        print!("Iterações {}\r",k);
    }

    println!("Iterações {}",k);

    Ok(dx)
}

fn main() {
    let f: Função = |x| x.powi(2) + 3.0 * x + 1.0;
    let x = 10000.0;
    let epsilon = 1e-10;

    match newton(&f, epsilon, x) {
        Ok(r) => {
            let corta = (r / epsilon).trunc() * epsilon;
            println!("A raiz encontrada é {}", corta);
            println!("F({}) é {}", corta, f(corta))
        }
        Err(Erro::DNaN) => println!("Erro ao calcular f(x)/f'(x)"),
    }
}
