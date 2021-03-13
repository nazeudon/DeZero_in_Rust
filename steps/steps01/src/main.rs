use ndarray::prelude::*;

fn main() {
    let data = arr1(&[1.0]);
    let x = Variable { data };
    println!("{:?}", x);
    println!("{:?}", x.data);

    let data = arr1(&[10.0]);
    let x = Variable { data };
    let f = Square {};
    let y = f.call(x);
    println!("{:?}", y);
    println!("{:?}", y.data);
}

#[derive(Debug)]
struct Variable<T> {
    data: T,
}

trait Function<T> {
    fn call(&self, input: Variable<T>) -> Variable<T> {
        let x = input.data;
        let data = self.forward(x);
        let output = Variable { data };
        output
    }

    fn forward(&self, x: T) -> T;
}

struct Square {}

impl Function<Array1<f32>> for Square {
    fn forward(&self, x: Array1<f32>) -> Array1<f32> {
        &x * &x
    }
}
