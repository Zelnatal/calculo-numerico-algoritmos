use std::rc::Rc;

struct Tabela<const LEN: usize> {
    x: [f64; LEN],
    y: [f64; LEN],
}

impl<const LEN: usize> Tabela<LEN> {
    #[allow(dead_code)]
    fn from_array(x: [f64; LEN], y: [f64; LEN]) -> Self {
        Self {
            x: x.clone(),
            y: y.clone(),
        }
    }

    #[allow(dead_code)]
    fn from_func(x: [f64; LEN], f: fn(f64) -> f64) -> Self {
        Self { x: x, y: x.map(f) }
    }
}

struct Lagrange<const LEN: usize> {
    t: Rc<Tabela<LEN>>,
    l: [Box<dyn Fn(f64) -> f64>; LEN],
}

impl<const LEN: usize> Lagrange<LEN> {
    fn new(t: Tabela<LEN>) -> Self {
        let t = Rc::new(t);
        let l = std::array::from_fn(|i| {
            let d =
                (0..LEN).filter_map(|j|{
                    if i == j {
                        None
                    } else {
                        Some(t.x[i] - t.x[j])
                    }
                }).product::<f64>();
            Box::new(Self::gerar_l(i, d, t.clone())) as Box<dyn Fn(f64) -> f64>
        });
        Self { t: t, l: l }
    }

    fn gerar_l(i: usize, d: f64, t: Rc<Tabela<LEN>>) -> impl Fn(f64) -> f64 {
        move |x| {
            (0..LEN)
                .filter_map(|j| if j == i { None } else { Some(x - t.x[j]) })
                .product::<f64>()
                / d
        }
    }

    fn calcular(&self, x: f64) -> f64 {
        (0..LEN).map(|i| self.t.y[i] * self.l[i](x)).sum::<f64>()
    }
}

fn main() {
    let tabela = Tabela::from_func([-1.0,0.0,1.0], |x| (-x).exp());
    let x = 5.0;
    let lagrande = Lagrange::new(tabela);
    println!("O g({}) = {}",x,lagrande.calcular(x))
    
}
