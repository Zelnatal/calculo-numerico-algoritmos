fn diagonal_dominante(a: &Vec<Vec<f64>>) -> bool {
    if a.len() != 0 && a.len() != a[0].len() {
        panic!("A matriz A está vazia ou não é quadrada");
    }
    a.iter().enumerate().all(|(i, r)| {
        let d = r[i].abs();
        d > r.iter().map(|n| n.abs()).sum::<f64>() - d
    })
}

fn norma(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    if a.len() != b.len() {
        panic!("Erro na norma, tamanhos diferentes")
    }
    let mut d: f64 = 0.0;
    for n in (0..a.len()).map(|i| (b[i] - a[i]).abs()) {
        d = d.max(n)
    }
    let mut m: f64 = 0.0;
    for &n in b.iter() {
        m = m.max(n.abs())
    }
    d / m
}

fn gauss_seidel(
    a: &Vec<Vec<f64>>,
    b: &Vec<f64>,
    mut x: Vec<f64>,
    epsilon: f64,
) -> Result<Vec<f64>, &'static str> {
    if a.len() != 0 && a.len() != a[0].len() {
        return Err("A matriz A está vazia ou não é quadrada");
    }
    if a.len() != b.len() {
        return Err("A matriz A tem tamanho diferente do Vetor b");
    }

    if a.len() != x.len() {
        return Err("A matriz A tem tamanho diferente do Vetor x");
    }

    if !diagonal_dominante(a) {
        return Err("A matriz A não tem a diagonal dominante");
    }

    let c = {
        let mut tmp = vec![vec![0.0; a.len()]; a.len()];
        for i in 0..a.len() {
            for j in 0..a.len() {
                if i == j {
                    continue;
                } else {
                    tmp[i][j] = -a[i][j] / a[i][i]
                }
            }
        }
        tmp
    };

    let g = {
        let mut tmp = vec![0.0; a.len()];
        for i in 0..a.len() {
            tmp[i] = b[i] / a[i][i]
        }
        tmp
    };
    let mut i = 1;
    loop {
        let mut xp = vec![0.0; a.len()];
        for i in 0..a.len() {
            xp[i] = g[i];
            for j in 0..a.len() {
                if i == j {
                    continue;
                } else if j < i {
                    xp[i] += c[i][j] * xp[j];
                } else {
                    xp[i] += c[i][j] * x[j];
                }
            }
        }

        if norma(&x, &xp) < epsilon {
            println!("Iterações {i}");
            break Ok(xp);
        }
        x = xp;
        print!("Iterações {i}\r");
        i += 1;
    }
}

fn main() {
    let a = vec![
        vec![10.0, 2.0, 1.0],
        vec![1.0, 5.0, 1.0],
        vec![2.0, 3.0, 10.0],
    ];
    let b = vec![7.0, -8.0, 6.0];
    let x = vec![0.0, 0.0, 0.0];
    let epsilon = 1e-16;

    match gauss_seidel(&a, &b, x, epsilon) {
        Ok(xs) => {
            println!("Os x encontrados:");
            for x in xs {
                println!("{x}");
            }
        }
        Err(s) => println!("{s}"),
    }
}
