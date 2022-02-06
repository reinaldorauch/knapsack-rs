#[derive(Debug)]
struct Matrix {
    buf: Vec<usize>,
    rows: usize,
    cols: usize
}

impl Matrix {
    fn new (rows: usize, cols: usize) -> Self {
        Matrix {
            buf: vec![0; rows * cols],
            rows,
            cols
        }
    }

    fn set(self: &mut Self, x: usize, y: usize, val: usize) {
        self.buf[x * self.rows + y] = val;
    }

    fn get(self: &Self, x: usize, y: usize) -> usize {
        self.buf[x * self.rows + y]
    }
}


fn main() {
    let stdin = std::io::stdin();

    let mut input = String::new();

    stdin.read_line(&mut input).expect("Could not read the capacity");
    let cap: usize = input.trim().parse().expect("Could not parse the capacity into usize");
    input.clear();
    
    stdin.read_line(&mut input).expect("Could not read the values of the items");
    let valor: Vec<usize> = input.trim().split(",").map(|v| v.trim().parse().expect("Could not parse the value into usize")).collect();
    input.clear();
    
    stdin.read_line(&mut input).expect("Could not read the weights of the items");
    let peso: Vec<usize> = input.trim().split(",").map(|v| v.trim().parse().expect("Could not parse the weight into usize")).collect();
    input.clear();

    assert_eq!(valor.len(), peso.len());

    println!("{}", mochila(cap, &peso, &valor, valor.len()));
}

fn mochila(cap: usize, peso: &Vec<usize>, valor: &Vec<usize>, total: usize) -> usize {
    let mut results = Matrix::new(cap + 1, total + 1);

    println!("{:?}", results);
    
    for i in 1..(total + 1) {
        for c in 1..(cap + 1) {
            let new_val = if peso[i - 1] <= c {
                let with_item = valor[i - 1] + results.get(i - 1, c - peso[i - 1]);
                let without_item = results.get(i - 1, c);
                max(with_item, without_item)
            } else {
                results.get(i - 1, c)
            };

            results.set(i, c, new_val);
        }
    }

    println!("{:?}", results);
    
    results.get(total, cap)
}

fn max<T: std::cmp::PartialOrd>(v1: T, v2: T) -> T {
    if v1 >= v2 {
        v1
    } else {
        v2
    }
}

#[test]
fn test_max() {
    assert_eq!(max(1, 2), 2);
    assert_eq!(max(2, 1), 2);
    assert_eq!(max(-1, 2), 2);
    assert_eq!(max(1321231213123i64,132910321921), 1321231213123i64);
    assert_eq!(max(1231231, 1321231213123i64), 1321231213123i64);
}
