#[derive(Debug, Clone)]
struct Matriz {
    matriz: Vec<Vec<f64>>,
    b: Vec<f64>,
}

#[derive(Debug)]
enum Error {
    NãoQuadrada,
    NãoTemTriangular,
    ColunaLenDiferenteB,
    DivisãoZero,
}

impl Matriz {
    fn new(matriz: Vec<Vec<f64>>, b: Vec<f64>) -> Self {
        if matriz.len() != 0 && matriz[0].len() != b.len() {
            panic!("A coluna da matriz tem que ter o mesmo tamanho de b")
        }
        Self { matriz, b }
    }

    fn m(&self) -> usize {
        self.matriz.len()
    }

    fn n(&self) -> usize {
        if self.m() == 0 {
            0
        } else {
            self.matriz[0].len()
        }
    }

    fn b_len(&self) -> usize {
        self.b.len()
    }

    fn converter_em_triangular_superior(&self) -> Result<Self, Error> {
        if self.m() != self.n() {
            return Err(Error::NãoQuadrada);
        }
        if self.n() != self.b_len() {
            return Err(Error::ColunaLenDiferenteB);
        }

        let mut retorno = self.clone();
        let matriz = &mut retorno.matriz;
        let b = &mut retorno.b;
        let mut k = 0;
        'k: while k < matriz.len() - 1 {
            if matriz[k][k] == 0.0 {
                for i in k + 1..matriz.len() {
                    if matriz[i][k] != 0.0 {
                        matriz.swap(i, k);
                        b.swap(i, k);
                        continue 'k;
                    }
                }
                return Err(Error::NãoTemTriangular);
            }
            for i in k + 1..matriz.len() {
                let mul = matriz[i][k] / matriz[k][k];
                matriz[i][k] = 0.0;
                for j in k + 1..matriz.len() {
                    matriz[i][j] -= mul * matriz[k][j];
                }
                b[i] -= mul * b[k];
            }
            k += 1
        }
        Ok(retorno)
    }
}

fn gauss(matriz: &Matriz) -> Result<Vec<f64>, Error> {
    let triangular = matriz.converter_em_triangular_superior()?;
    if triangular.b_len() == 0 {
        return Ok(vec![]);
    }
    let b = &triangular.b;
    let m = &triangular.matriz;

    let mut xs = vec![0.0; matriz.b_len()];
    for i in (0..b.len()).rev() {
        if m[i][i] == 0.0 {
            return Err(Error::DivisãoZero);
        }
        let mut s = 0.0;
        for j in i + 1..b.len() {
            s += m[i][j] * xs[j]
        }
        xs[i] = (b[i] - s) / m[i][i];
    }
    Ok(xs)
}

fn main() {
    let m = vec![
        vec![0.0, 2.0, 4.0],
        vec![1.0, 2.0, 2.0],
        vec![4.0, 3.0, 0.0],
    ];
    let b = vec![1.0, 2.0, 3.0];
    let matriz = Matriz::new(m, b);
    match gauss(&matriz) {
        Ok(xs) => {
            println!("Os xs encontrados são :");
            for x in xs {
                println!("{}", x)
            }
        }
        Err(Error::NãoQuadrada) => println!("A matriz não é quadrada"),
        Err(Error::ColunaLenDiferenteB) => {
            println!("O tamanho da coluna da matriz é diferente do tamanho de b")
        }
        Err(Error::NãoTemTriangular) => println!("Não foi possível converte em triangular"),
        Err(Error::DivisãoZero) => println!("Teve uma divisão por zero"),
    }
}
