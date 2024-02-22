#[derive(Debug)]
struct Cholesky {
    l: Vec<Vec<f64>>,
}

impl Cholesky {
    fn new(a: &Vec<Vec<f64>>) -> Result<Self, &'static str> {
        if a.len() == 0 {
            return Err("Matriz A está vazia");
        }
        if a[0].len() != a.len() {
            return Err("Matriz A não é quadrada");
        }
        if !a.iter().enumerate().all(|(i, row)| {
            row.iter()
                .enumerate()
                .skip(i + 1)
                .all(|(j, &v)| v == a[j][i])
        }) {
            return Err("Matriz A não é simétrica");
        }

        let mut l = vec![vec![0.0; a.len()]; a.len()];

        for i in 0..a.len() {
            let s = l[i].iter().take(i).map(|n: &f64| n.powi(2)).sum::<f64>();
            l[i][i] = (a[i][i] - s).sqrt();
            if l[i][i].is_nan() || l[i][i] == 0.0 {
                return Err("O l[i][i] é NaN ou Zero");
            }
            for j in i + 1..a.len() {
                let s = (0..i.min(j))
                    .fold(0.0, |acc, k| acc + (l[i][k] * l[k][j]));
                l[i][j] = (a[i][j] - s) / l[i][i];
                l[j][i] = l[i][j]
            }
        }

        Ok(Self { l })
    }
}

fn cholesky(a: &Vec<Vec<f64>>, b: &Vec<f64>) -> Result<Vec<f64>, &'static str> {
    let Cholesky { l } = Cholesky::new(a)?;
    if a.len() != b.len() {
        return Err("O tamanho de B é diferente do tamanho de A");
    }

    let mut ys = vec![0.0; a.len()];
    for i in 0..a.len() {
        let mut s = 0.0;
        for j in 0..i {
            s += l[i][j] * ys[j];
        }
        ys[i] = (b[i] - s) / l[i][i];
    }

    let mut xs = vec![0.0; a.len()];
    for i in (0..a.len()).rev() {
        let mut s = 0.0; 
        for j in i + 1..a.len() {
            s += l[i][j] * xs[j]
        }
        xs[i] = (ys[i] - s) / l[i][i];
    }

    Ok(xs)
}

fn main() {
    let a = vec![
        vec![4.0, -2.0, 2.0],
        vec![-2.0, 10.0, -7.0],
        vec![2.0, -7.0, 30.0],
    ];
    let b = vec![8.0, 11.0, -31.0];

    match cholesky(&a, &b) {
        Ok(xs) => {
            println!("Os xs encontrados são :");
            for x in xs {
                println!("{}", x)
            }
        }
        Err(m) => println!("{}", m),
    }
}
