use ndarray::prelude::*;
use num_traits::{Float, FromPrimitive};

#[derive(Debug, Clone, Copy)]
struct Variable<T> {
    data: T,
    grad: Option<T>,
}
impl<T> Variable<T> {
    fn new(data: T) -> Variable<T> {
        let grad: Option<T> = None;
        Variable { data, grad }
    }
}

#[derive(Debug)]
struct Square<T> {
    input: T,
}
#[derive(Debug)]
struct Exp<T> {
    input: T,
}

trait Function<T> {
    fn new(input: Variable<T>) -> Self;
    fn input(&self) -> Variable<T>;
    fn forward(&self, x: T) -> T;
    fn backward(&self, gy: T) -> T;
    fn call(&self) -> Variable<T> {
        let x = self.input().data;
        let data = self.forward(x);
        let output = Variable::new(data);
        output
    }
}

impl Function<Array1<f32>> for Square<Variable<Array1<f32>>> {
    fn new(input: Variable<Array1<f32>>) -> Square<Variable<Array1<f32>>> {
        Square { input }
    }
    fn input(&self) -> Variable<Array1<f32>> {
        self.input.clone()
    }
    fn forward(&self, x: Array1<f32>) -> Array1<f32> {
        x.pow(2.0)
    }
    fn backward(&self, gy: Array1<f32>) -> Array1<f32> {
        let x = self.input().data;
        let gx = 2.0 * x * gy;
        gx
    }
}

impl Function<Array1<f32>> for Exp<Variable<Array1<f32>>> {
    fn new(input: Variable<Array1<f32>>) -> Exp<Variable<Array1<f32>>> {
        Exp { input }
    }
    fn input(&self) -> Variable<Array1<f32>> {
        self.input.clone()
    }
    fn forward(&self, x: Array1<f32>) -> Array1<f32> {
        x.exp()
    }
    fn backward(&self, gy: Array1<f32>) -> Array1<f32> {
        let x = self.input().data;
        let gx = x.exp() * gy;
        gx
    }
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

fn main() {
    let data = arr1(&[1.0]);
    let x = Variable::new(data);
    println!("x        : {:?}", x);
    println!("x.data   : {:?}", x.data);
    println!("x.data[0]: {:?}", x.data[0]);
    println!("");

    let data = arr1(&[10.0]);
    let x = Variable::new(data);
    let mut f = Square::new(x);
    let y = f.call();
    println!("y        : {:?}", y);
    println!("y.data   : {:?}", y.data);
    println!("y.data[0]: {:?}", y.data[0]);
    f.input.grad = Some(f.backward(arr1(&[1.0])));
    println!("f.input.data: {:?}", f.input.data);
    println!("f.input.grad: {:?}", f.input.grad);
    println!("");

    let data = arr1(&[10.0]);
    let x = Variable::new(data);
    let mut f = Exp::new(x);
    let y = f.call();
    println!("y        : {:?}", y);
    println!("y.data   : {:?}", y.data);
    println!("y.data[0]: {:?}", y.data[0]);
    f.input.grad = Some(f.backward(arr1(&[1.0])));
    println!("f.input.data: {:?}", f.input.data);
    println!("f.input.grad: {:?}", f.input.grad);
    println!("");
}
