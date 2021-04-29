use ndarray::prelude::*;
use num_traits::{Float, FromPrimitive};

#[derive(Debug)]
struct Variable<T> {
    data: T,
}

trait MathFunc<T> {
    fn exp(&self) -> Self;
    fn pow(&self, p: T) -> Self;
}

impl<T> MathFunc<T> for Array1<T>
where
    T: Float + FromPrimitive,
{
    fn exp(&self) -> Self {
        self.map(|&v| v.exp())
    }
    fn pow(&self, p: T) -> Self {
        self.map(|v| v.powf(p))
    }
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

struct Pow;

impl Function<Array1<f32>> for Pow {
    fn forward(&self, x: Array1<f32>) -> Array1<f32> {
        x.pow(2.0)
    }
}

struct Exp;

impl Function<Array1<f32>> for Exp {
    fn forward(&self, x: Array1<f32>) -> Array1<f32> {
        x.exp()
    }
}

fn main() {
    let data = arr1(&[1.0]);
    let x = Variable { data };
    println!("{:?}", x);
    println!("{:?}", x.data);
    println!("{:?}", x.data[0]);

    let data = arr1(&[10.0]);
    let x = Variable { data };
    let f = Pow {};
    let y = f.call(x);
    println!("{:?}", y);
    println!("{:?}", y.data);
    println!("{:?}", y.data[0]);

    let data = arr1(&[10.0]);
    let x = Variable { data };
    let f = Exp {};
    let y = f.call(x);
    println!("{:?}", y);
    println!("{:?}", y.data);
    println!("{:?}", y.data[0]);
}
