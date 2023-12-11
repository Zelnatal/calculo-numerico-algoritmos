type Função = fn(f64) -> f64;

enum Erro {
    IntervaloInvalido,
    Desconhecido(String),
}

fn bisseção(f: &Função, epsilon: f64, mut a: f64, mut b: f64) -> Result<f64, Erro> {
    let mut fa = f(a);
    let mut fb = f(b);

    if fa * fb > 0.0 {
        return Err(Erro::IntervaloInvalido);
    }

    while (a - b).abs() >= epsilon && fa.abs() >= epsilon && fb.abs() >= epsilon {
        let x = (a + b) / 2.0;
        let fx = f(x);

        if fx * fa < 0.0 {
            b = x;
            fb = fx;
            continue;
        }

        if fx * fb < 0.0 {
            a = x;
            fa = fx;
            continue;
        }

        return Err(Erro::Desconhecido("Erro dentro do loop".to_string()));
    }

    if (a - b).abs() < epsilon || fa.abs() < epsilon {
        return Ok(a);
    }

    if fb.abs() < epsilon {
        return Ok(b);
    }

    Err(Erro::Desconhecido(
        "Chegou ao final sem encontrar a raiz".to_string(),
    ))
}

fn main() {
    let f: Função = |x| x.sin() * x.powf(x);
    let epsilon = 10.0_f64.powi(-15);
    let a = 6.0;
    let b = 7.0;

    match bisseção(&f, epsilon, a, b) {
        Ok(resultado) => {
            println!("A raiz encontrada é {}", resultado);
            println!("F({}) é {}", resultado, f(resultado))
        }
        Err(Erro::IntervaloInvalido) => println!("O intervalo precisa cruza o eixo x"),
        Err(Erro::Desconhecido(mensagem)) => println!("Deu Erro: {}", mensagem),
    }
}
