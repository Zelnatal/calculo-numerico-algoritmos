type Função = fn(f64) -> f64;

enum Error {
    Zero,
    Igual,
}

fn secante(f: &Função, epsilon: f64, mut a: f64, mut b: f64) -> Result<f64, Error> {
    if a == b {
        return Err(Error::Igual);
    }
    let mut fa = f(a);
    let mut fb = f(b);
    if fa.abs() < epsilon {
        println!("Iterações 0");
        Ok(a)
    } else if fb.abs() < epsilon {
        println!("Iterações 0");
        Ok(b)
    } else {
        let mut i = 0;
        let mut c;
        loop {
            i += 1;
            c = b - (fb / ((fb - fa) / (b - a)));
            if c.is_nan() {
                println!("Iterações {}", i);
                break Err(Error::Zero);
            }

            (a, b) = (b, c);
            (fa, fb) = (f(a), f(b));

            if (a - b).abs() < epsilon || fa.abs() < epsilon {
                println!("Iterações {}", i);
                break Ok(a);
            }

            if fb.abs() < epsilon {
                println!("Iterações {}", i);
                break Ok(b);
            }
            print!("Iterações {}\r", i);
        }
    }
}

fn main() {
    let f: Função = |x| x.powi(2) + 3.0 * x + 1.0;
    let a = 10.0;
    let b = -0.3819660111;
    let epsilon = 1e-10;

    match secante(&f, epsilon, a, b) {
        Ok(r) => {
            let corta = (r / epsilon).trunc() * epsilon;
            println!("A raiz encontrada é {}", corta);
            println!("F({}) é {}", corta, f(corta))
        }
        Err(Error::Igual) => println!("Os dois pontos não podem ser iguais"),
        Err(Error::Zero) => println!("Teve uma divisão por zero"),
    }
}
