#[derive(Debug, Clone)]
struct SistemaEquação {
    a: Vec<Vec<f64>>,
    b: Vec<f64>,
}

#[derive(Debug)]
enum Error {
    NãoQuadrada,
    AVazio,
    LenColunaADiferenteB,
    NãoPossível,
}

impl SistemaEquação {
    fn new(a: Vec<Vec<f64>>, b: Vec<f64>) -> Self {
        Self { a, b }
    }

    fn preparar_sistema_triangular_inferior(&self) -> Result<Self, Error> {
        if self.a.len() == 0 {
            return Err(Error::AVazio);
        }
        if self.a.len() != self.a[0].len() {
            return Err(Error::NãoQuadrada);
        }
        if self.a[0].len() != self.b.len() {
            return Err(Error::LenColunaADiferenteB);
        }

        let mut retorno = self.clone();
        let mut k = retorno.a.len() - 1;
        'k: loop {
            if retorno.a[k][k] == 0.0 {
                for i in (0..k).rev() {
                    if retorno.a[i][k] != 0.0 {
                        retorno.a.swap(i, k);
                        retorno.b.swap(i, k);
                        continue 'k;
                    }
                }
                return Err(Error::NãoPossível);
            }
            for i in 0..k {
                let mul = retorno.a[i][k] / retorno.a[k][k];
                retorno.a[i][k] = 0.0;
                for j in 0..k {
                    retorno.a[i][j] -= mul * retorno.a[k][j];
                }
                retorno.b[i] -= mul * retorno.b[k];
            }
            match k.checked_sub(1) {
                Some(n) => k = n,
                None => break,
            }
        }
        Ok(retorno)
    }

    fn espacidade(&self) -> usize {
        self.a.iter().fold(0, |acc, m| {
            acc + m
                .iter()
                .fold(0, |acc, n| if *n == 0.0 { acc + 1 } else { acc })
        })
    }
}

fn gauss(sistema: &SistemaEquação) -> Result<Vec<f64>, Error> {
    let sistema = sistema.preparar_sistema_triangular_inferior()?;
    println!("A espacidade é {}",sistema.espacidade());
    let mut xs = vec![0.0; sistema.b.len()];
    for i in 0..sistema.b.len() {
        let mut s = 0.0;
        for j in 0..i {
            s += sistema.a[i][j] * xs[j]
        }
        xs[i] = (sistema.b[i] - s) / sistema.a[i][i];
    }
    Ok(xs)
}

fn main() {
    let a = vec![
        vec![3.0, 2.0, 4.0],
        vec![1.0, 1.0, 2.0],
        vec![4.0, 3.0, -2.0],
    ];
    let b = vec![1.0, 2.0, 3.0];
    let sistema = SistemaEquação::new(a, b);
    match gauss(&sistema) {
        Ok(xs) => {
            println!("Os xs encontrados são :");
            for x in xs {
                println!("{}", x)
            }
        }
        Err(Error::AVazio) => println!("O matriz A não pode está vazio"),
        Err(Error::NãoQuadrada) => println!("A matriz A não é quadrada"),
        Err(Error::LenColunaADiferenteB) => {
            println!("O tamanho da coluna da matriz A é diferente do tamanho de Vetor B")
        }
        Err(Error::NãoPossível) => {
            println!("Não foi possível resolver pelo matriz triangular superior")
        }
    }
}
