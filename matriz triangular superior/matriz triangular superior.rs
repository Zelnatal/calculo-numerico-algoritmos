#[derive(Debug,Clone)]
struct Matriz {
    matriz: Vec<Vec<f64>>,
    b: Vec<f64>
}

#[derive(Debug)]
enum Error {
    NãoQuadrada,
    NãoTemTriangular,
    ColunaLenDiferenteB,
}

impl Matriz {

    fn new(matriz: Vec<Vec<f64>>, b: Vec<f64>) -> Self {
        if matriz.len() != 0 && matriz[0].len() != b.len(){
            panic!("A coluna da matriz tem que ter o mesmo tamanho de b")
        }
        Self {
            matriz,
            b
        }
    }

    fn m(&self) -> usize {
        self.matriz.len()
    }

    fn n(&self) -> usize {
        if self.m() == 0{
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
        'k: while k < matriz.len()-1{
            if matriz[k][k] == 0.0 {
                for i in k+1 .. k {
                    if matriz[i][k] != 0.0 {
                        matriz.swap(i, k);
                        continue 'k;
                    }
                }
                return Err(Error::NãoTemTriangular);
            }
            for i in k+1..matriz.len() {
                let mul = matriz[i][k]/matriz[k][k];
                matriz[i][k] = 0.0;
                for j in k+1 .. matriz.len() {
                    matriz[i][j] -= mul*matriz[k][j];
                }
                b[i] -= mul*b[k];
            }
            k += 1

        }
        Ok(retorno)
    } 
}

fn main() {
    let m = vec![vec![3.0,2.0,4.0],vec![1.0,1.0,2.0],vec![4.0,3.0,-2.0]];
    let b = vec![1.0,2.0,3.0];
    let matriz = Matriz::new(m, b);
    println!("{:#?}",matriz.converter_em_triangular_superior())
}
