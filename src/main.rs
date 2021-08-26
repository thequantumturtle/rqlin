#![allow(mixed_script_confusables)]
pub mod complex_vector;
pub mod complex_number;
use crate::complex_number::ComplexNumber;

fn main() {
    let c = ComplexNumber::new(2.0, 1.0);
    println!("{}", c);

    let mut x_vec:Vec<ComplexNumber> = Vec::new();
    let mut y_vec:Vec<ComplexNumber> = Vec::new();
    let a = ComplexNumber::new(1.0, 2.0);
    let b = ComplexNumber::new(3.0, -4.0);
    let c = ComplexNumber::new(-6.0, -7.0);
    x_vec.push(a);
    y_vec.push(b);
    x_vec.push(c);

    let z_vec = complex_vector::add(&x_vec, &y_vec);
    println!("{:?}", x_vec);
    println!("{:?}", y_vec);
    println!("{:?}", z_vec);

    println!("{:?}", complex_vector::inverse(&x_vec));
    println!("{:?}", complex_vector::add(&x_vec, &complex_vector::inverse(&x_vec)));

    let a = ComplexNumber::new(1.0, 2.0);
    println!("{:?}", complex_vector::scalar_mul(&a, &x_vec));
}
