struct Tabela {
    x: Box<[f64]>,
    y: Box<[f64]>,
}

impl Tabela {
    #[allow(dead_code)]
    fn from_array(x: &[f64], y: &[f64]) -> Self {
        if x.len() != y.len() {
            panic!("O array x tem tamanho diferente do y")
        }
        if x.len() == 0 {
            panic!("O array está vazio")
        }
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    #[allow(dead_code)]
    fn from_func(x: &[f64], f: fn(f64) -> f64) -> Self {
        if x.len() == 0 {
            panic!("O array está vazio")
        }
        Self {
            x: x.into(),
            y: x.iter().map(|&x| f(x)).collect(),
        }
    }
}

struct Newton {
    f: Box<dyn Fn(f64) -> f64>,
}

impl Newton {
    fn new(t: &Tabela) -> Self {
        let mut tri_d = (1..t.x.len() + 1)
            .rev()
            .map(|i| Box::new([0.0].repeat(i)))
            .collect::<Box<_>>();
        for (i, &v) in t.y.iter().enumerate() {
            tri_d[0][i] = v;
        }

        for i in 1..t.x.len() {
            for j in 0..tri_d[i].len() {
                tri_d[i][j] = (tri_d[i - 1][j + 1] - tri_d[i - 1][j]) / (t.x[j + i] - t.x[j])
            }
        }

        let d = (0..t.x.len()).map(|i| tri_d[i][0]).collect::<Box<_>>();
        let xs = t.x[0..t.x.len() - 1]
            .iter()
            .take(t.x.len() - 1)
            .map(|&n| n)
            .collect::<Box<_>>();

        let f = Box::new(move |x: f64| -> f64 {
            let mut s = d[0];
            for i in 1..d.len() {
                let mut m = 1.0;
                for j in 0..i {
                    m *= x - xs[j];
                }
                s += d[i] * m;
            }
            s
        }) as Box<dyn Fn(f64) -> f64>;

        Self { f }
    }

    fn calcular(&self, x: f64) -> f64 {
        (self.f)(x)
    }
}

fn main() {
    let tabela = Tabela::from_func(&[-1.0,0.0,1.0], |x| (-x).exp());
    let x = [5.0,-5.0,10.0,1.1,1000.0];

    let newton = Newton::new(&tabela);

    for x in x {
        println!("O g({}) = {}",x,newton.calcular(x));
    }
}
