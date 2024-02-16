use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    fn pow2(self) -> Self {
        self * self
    }

    fn sqrt(self) -> Self {
        if self.im == 0.0 {
            if self.re >= 0.0 {
                Self {
                    re: self.re.sqrt(),
                    im: self.im,
                }
            } else {
                Self {
                    re: 0.0,
                    im: (-self.re).sqrt(),
                }
            }
        } else {
            let m = (self.re.powi(2) + self.im.powi(2)).sqrt();
            Self {
                re: ((m + self.re) / 2.0).sqrt(),
                im: (self.im / self.im.abs()) * ((m - self.re) / 2.0).sqrt(),
            }
        }
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Self::Output {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl AddAssign<Complex> for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl Sum<Complex> for Complex {
    fn sum<I: Iterator<Item = Complex>>(iter: I) -> Self {
        iter.fold(Complex::new(0.0, 0.0), |acc, n| acc + n)
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Self::Output {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Sub<Complex> for f64 {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Self::Output {
            re: self - rhs.re,
            im: -rhs.im,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Self::Output {
        Self::Output {
            re: (self.re * rhs.re) - (self.im * rhs.im),
            im: (self.re * rhs.im) + (self.im * rhs.re),
        }
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Self::Output {
        let d = rhs.re.powi(2) + rhs.im.powi(2);
        Self::Output {
            re: ((self.re * rhs.re) + (self.im * rhs.im)) / d,
            im: ((self.im * rhs.re) + (self.re * rhs.im)) / d,
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im == 0.0 {
            write!(f, "{}", self.re)
        } else {
            write!(f, "{} + {}i", self.re, self.im)
        }
    }
}

#[derive(Debug)]
struct Cholesky {
    l: Vec<Vec<Complex>>,
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

        let mut l = vec![vec![Complex::new(0.0, 0.0); a.len()]; a.len()];

        for i in 0..a.len() {
            let s = l[i].iter().take(i).map(|n| n.pow2()).sum::<Complex>();
            l[i][i] = (a[i][i] - s).sqrt();
            if l[i][i] == Complex::new(0.0, 0.0) {
                return Err("O l[i][i] é NaN ou Zero");
            }
            for j in i + 1..a.len() {
                let s =
                    (0..i.min(j)).fold(Complex::new(0.0, 0.0), |acc, k| acc + (l[i][k] * l[k][j]));
                l[i][j] = (a[i][j] - s) / l[i][i];
                l[j][i] = l[i][j]
            }
        }

        Ok(Self { l })
    }
}

fn cholesky(a: &Vec<Vec<f64>>, b: &Vec<f64>) -> Result<Vec<Complex>, &'static str> {
    let Cholesky { l } = Cholesky::new(a)?;
    if a.len() != b.len() {
        return Err("O tamanho de B é diferente do tamanho de A");
    }

    let mut ys = vec![Complex::new(0.0, 0.0); a.len()];
    for i in 0..a.len() {
        let mut s = Complex::new(0.0, 0.0);
        for j in 0..i {
            s += l[i][j] * ys[j];
        }
        ys[i] = (b[i] - s) / l[i][i];
    }

    let mut xs = vec![Complex::new(0.0, 0.0); a.len()];
    for i in (0..a.len()).rev() {
        let mut s = Complex::new(0.0, 0.0);
        for j in i + 1..a.len() {
            s += l[i][j] * xs[j]
        }
        xs[i] = (ys[i] - s) / l[i][i];
    }

    Ok(xs)
}

fn main() {
    let a = vec![
        vec![1.0, 2.0, 3.0],
        vec![2.0, 6.0, 4.0],
        vec![3.0, 4.0, 10.0],
    ];
    let b = vec![4.0, 5.0, 5.0];

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
