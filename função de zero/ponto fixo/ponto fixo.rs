use std::io::{self, Write};

type Função = fn(f64) -> f64;

enum Erro {
    NaNG,
    NaNF,
}

fn ponto_fixo(f: &Função, g: &Função, mut x: f64, epsilon: f64) -> Result<f64, Erro> {
    let mut gx = g(x);
    let mut fx = f(x);

    if gx.is_nan() {
        return Err(Erro::NaNG);
    }

    if fx.is_nan() {
        return Err(Erro::NaNG);
    }

    let mut k = 0;
    while fx.abs() >= epsilon && (x - gx).abs() >= epsilon {
        x = gx;
        gx = g(x);
        fx = f(x);

        if gx.is_nan() {
            return Err(Erro::NaNG);
        }

        if fx.is_nan() {
            return Err(Erro::NaNF);
        }
        
        k += 1;
        print!("{} iterações\r", k);
        io::stdout().flush().ok();
    }

    println!("{} iterações", k);
    return Ok(x);
}

fn main() {
    let f: Função = |x| x.powi(2) + x - 6.0;
    let g: Função = |x| (6.0 - x).sqrt();
    let epsilon = 10.0_f64.powi(-3);
    let x = -4.0;

    match ponto_fixo(&f, &g, x, epsilon) {
        Ok(raiz) => {
            let corta = (raiz / epsilon).trunc() * epsilon;
            println!("A raiz encontrada é {}", corta);
            println!("F({}) = {}", corta, f(corta));
        }
        Err(Erro::NaNG) => println!("O g(x) retornou um NaN"),
        Err(Erro::NaNF) => println!("O f(x) retornou um NaN")
    }
}
