use std::collections::VecDeque;

type Função = fn(f64) -> f64;

enum Erro {
    IntervaloInvalido,
    IteraçõesMáxima(u32),
    Desconhecido(String),
}

fn iter_máximo(a: f64, b: f64, epsilon: f64) -> u32 {
    return ((b - a).abs().log2() - epsilon.log2()).ceil() as u32;
}

fn falsa_posição(
    f: &Função,
    epsilon: f64,
    mut a: f64,
    mut b: f64,
    mut k_atual: u32,
    k_máximo: u32,
) -> Result<f64, Erro> {
    let mut fa = f(a);
    let mut fb = f(b);

    if fa * fb > 0.0 {
        return Err(Erro::IntervaloInvalido);
    }

    while k_atual <= k_máximo
        && (a - b).abs() >= epsilon
        && fa.abs() >= epsilon
        && fb.abs() >= epsilon
    {
        k_atual += 1;
        let x = (a*fb - b*fa) / (fb-fa);
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

    println!("{}",k_atual);

    if k_atual > k_máximo {
        return Err(Erro::IteraçõesMáxima(k_máximo));
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

fn encontrar_intervalo(f: &Função, a: f64, b: f64, k_máximo: u32) -> Option<(f64, f64, u32)> {
    let fa = f(a);
    let fb = f(b);
    let k_atual: u32 = 0;

    if fa * fb < 0.0 {
        return Some((a, b, k_atual));
    }

    let mut fila: VecDeque<(f64, f64, u32)> = VecDeque::new();
    fila.push_front((a, b, k_atual));

    while fila.len() > 0 {
        let (aa, ba, ka) = fila.pop_back().unwrap();
        if ka > k_máximo {
            continue;
        }

        let faa = f(aa);
        let fba = f(ba);

        let x = (aa*fba.abs() + ba*faa.abs()) / (fba.abs()+faa.abs());
        let fx = f(x);

        if fx * faa < 0.0 {
            return Some((aa, x, ka + 1));
        }

        if fx * fba < 0.0 {
            return Some((ba, x, ka + 1));
        }

        fila.push_front((aa, x, ka + 1));
        fila.push_front((ba, x, ka + 1));
    }

    None
}

fn main() {
    let f: Função = |x| x.powi(2) - 4.0;
    let epsilon = 10.0_f64.powi(-3);
    let a = -3.0;
    let b = 3.0;

    let k_máximo = iter_máximo(a, b, epsilon);
    println!("{}",k_máximo);

    if let Some((a_atual, b_atual, k_atual)) = encontrar_intervalo(&f, a, b, k_máximo) {
        match falsa_posição(&f, epsilon, a_atual, b_atual, k_atual, k_máximo) {
            Ok(resultado) => {
                let corta = (resultado / epsilon).trunc() * epsilon;
                println!("A raiz encontrada é {}", corta);
                println!("F({}) é {}", corta, f(corta))
            }
            Err(Erro::IntervaloInvalido) => println!("O intervalo precisa cruza o eixo x"),
            Err(Erro::IteraçõesMáxima(máximo)) => {
                println!("Ultrapassou o limite de iterações maxima de {}", máximo)
            }
            Err(Erro::Desconhecido(mensagem)) => println!("Deu Erro: {}", mensagem),
        }
    } else {
        println!("Não foi encontrado raiz nesse intervalo");
    }
}
