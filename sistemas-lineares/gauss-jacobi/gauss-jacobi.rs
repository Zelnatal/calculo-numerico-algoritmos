use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
struct Vetor(Vec<f64>);

impl Vetor {
    fn norma(&self) -> f64 {
        self.0.iter().map(|n| n.powi(2)).sum::<f64>().sqrt()
    }

    fn precisão(&self, other: &Vetor) -> f64 {
        (self.clone() - other.clone()).norma() / self.norma()
    }
}

impl Sub<Vetor> for Vetor {
    type Output = Vetor;

    fn sub(self, rhs: Vetor) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            panic!("Os vetores são de tamanhos diferentes");
        }
        let mut out = self;
        for (i, v) in out.0.iter_mut().enumerate() {
            *v -= rhs.0[i]
        }
        out
    }
}

impl Add<Vetor> for Vetor {
    type Output = Vetor;

    fn add(self, rhs: Vetor) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            panic!("Os vetores são de tamanhos diferentes");
        }
        let mut out = self;
        for (i, v) in out.0.iter_mut().enumerate() {
            *v += rhs.0[i]
        }
        out
    }
}

trait DiagonalDominante {
    fn diagonal_dominante(&self) -> bool;
}

impl DiagonalDominante for Vec<Vec<f64>> {
    fn diagonal_dominante(&self) -> bool {
        if self.len() != 0 && self.len() != self[0].len() {
            panic!("A matriz A está vazia ou não é quadrada");
        }
        self.iter().enumerate().all(|(i, r)| {
            let d = r[i].abs();
            d > r.iter().map(|n| n.abs()).sum::<f64>() - d
        })
    }
}

impl Mul<Vetor> for Vec<Vec<f64>> {
    type Output = Vetor;

    fn mul(self, rhs: Vetor) -> Self::Output {
        if self.len() != 0 && self.len() != self[0].len() {
            panic!("A matriz A está vazia ou não é quadrada");
        }
        if self.len() != rhs.0.len() {
            panic!("A matriz A tem tamanho diferente do Vetor b");
        }
        let mut out = vec![0.0; rhs.0.len()];
        for (i, v) in out.iter_mut().enumerate() {
            *v = rhs
                .0
                .iter()
                .enumerate()
                .map(|(j, n)| n * self[i][j])
                .sum::<f64>()
        }
        Vetor(out)
    }
}

fn gauss_jacobi(
    a: &Vec<Vec<f64>>,
    b: &Vec<f64>,
    x: &Vec<f64>,
    epsilon: f64,
) -> Result<Vec<f64>, &'static str> {
    if a.len() != 0 && a.len() != a[0].len() {
        return Err("A matriz A está vazia ou não é quadrada");
    }
    if a.len() != b.len() {
        return Err("A matriz A tem tamanho diferente do Vetor b");
    }

    if !a.diagonal_dominante() {
        return Err("A matriz A não tem a diagonal dominante");
    }

    let c = {
        let mut tmp = a.clone();
        for i in 0..a.len() {
            for j in 0..a.len() {
                if i == j {
                    tmp[i][j] = 0.0
                } else {
                    tmp[i][j] /= -a[i][i]
                }
            }
        }
        tmp
    };

    let g = {
        let mut tmp = Vetor(b.clone());
        for i in 0..b.len() {
            tmp.0[i] /= a[i][i]
        }
        tmp
    };

    let mut x = Vetor(x.clone());
    let mut i = 1;
    loop {
        let xp = c.clone() * x.clone() + g.clone();
        if xp.precisão(&x) < epsilon {
            println!("Iterações {i}");
            break Ok(xp.0);
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

    match gauss_jacobi(&a, &b, &x, epsilon) {
        Ok(xs) => {
            println!("Os x encontrados:");
            for x in xs {
                println!("{x}");
            }
        }
        Err(s) => println!("{s}"),
    }
}
