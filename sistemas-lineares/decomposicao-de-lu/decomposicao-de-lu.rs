type Matriz = Vec<Vec<f64>>;

#[derive(Debug)]
struct SistemaLu {
    l: Matriz,
    u: Matriz,
}

impl SistemaLu {
    fn new(a: &Matriz) -> Result<Self, &'static str> {
        if a.len() == 0 {
            return Err("Matriz A não pode está vazia");
        }

        if a.len() != a[0].len() {
            return Err("Matriz A tem que ser quadrada");
        }

        let mut l = vec![vec![0.0; a.len()]; a.len()];
        let mut u = l.clone();

        for i in 0..a.len() {
            l[i][i] = 1.0;
            for j in 0..a.len() {
                let mut s = 0.0;
                for k in 0..i.min(j) {
                    s += l[i][k] * u[k][j];
                }
                if j < i {
                    l[i][j] = (a[i][j] - s) / u[j][j];
                } else {
                    u[i][j] = a[i][j] - s; // l[i][i] = 1 então não precisa dividir
                }
            }
        }

        Ok(Self { l, u })
    }

    fn quantidade_zeros(m: &Matriz) -> usize {
        m.iter().fold(0, |acc, m| {
            acc + m
                .iter()
                .fold(0, |acc, n| if *n == 0.0 { acc + 1 } else { acc })
        })
    }
}

fn lu(a: &Matriz, b: &Vec<f64>) -> Result<Vec<f64>, &'static str> {
    let SistemaLu { l, u } = SistemaLu::new(a)?;
    println!("A quantidade de zeros de l é {}",SistemaLu::quantidade_zeros(&l));
    println!("A quantidade de zeros de u é {}",SistemaLu::quantidade_zeros(&u));
    if a.len() != b.len() {
        return Err("o tamanho da coluna de A tem que do mesmo tamanho de b");
    }

    let mut ys = vec![0.0; a.len()];
    for i in 0..a.len() {
        let mut s = 0.0;
        for j in 0..i {
            s += l[i][j] * ys[j]
        }
        ys[i] = b[i] - s; // l[i][i] = 1 então não precisa dividir
    }

    let mut xs = vec![0.0; a.len()];
    for i in (0..a.len()).rev() {
        let mut s = 0.0;
        for j in i + 1..a.len() {
            s += u[i][j] * xs[j]
        }
        xs[i] = (ys[i] - s) / u[i][i];
    }

    Ok(xs)
}

fn main() {
    let a = vec![
        vec![1.0, 43.0, 534.0, 654.0, 23.0],
        vec![4365.0, 45.0, 234.0, 543.0, 452.0],
        vec![325.0, -234.0, -4634.0, 234.0, 123.0],
        vec![2.0, 44.0, 6534.0, 23.0, 5.0],
        vec![5432.0, -234.0, -54.0, 345.0, 34.0],
    ];
    let b = vec![543.0, 123.0, 34.0, -2345.0, 23.0];

    match lu(&a, &b) {
        Ok(xs) => {
            println!("Os xs encontrados são :");
            for x in xs {
                println!("{}", x)
            }
        }
        Err(m) => println!("{}", m),
    }
}
