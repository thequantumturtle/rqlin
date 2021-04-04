struct ComplexNumber{
    real: f32,
    imaginary: f32,
}

fn cnum(r:f32, i:f32) -> ComplexNumber{
    ComplexNumber{
        real:r, 
        imaginary:i,
    }
}

fn cadd(a:&ComplexNumber, b:&ComplexNumber) -> ComplexNumber{
    let real = a.real + b.real;
    let imaginary = a.imaginary + b.imaginary;
    cnum(real, imaginary)
}

fn csub(a:&ComplexNumber, b:&ComplexNumber) -> ComplexNumber{
    let neg_b = cnum(-b.real, -b.imaginary);
    cadd(&a, &neg_b)
}

fn cmul(a:&ComplexNumber, b:&ComplexNumber) -> ComplexNumber{
    let real = a.real * b.real + a.imaginary * b.imaginary * -1.0;
    let imaginary = a.real * b.imaginary + a.imaginary * b.real;
    cnum(real, imaginary)
}

fn cdiv(a:&ComplexNumber, b:&ComplexNumber) -> ComplexNumber{
    let quotient = modulus(&b).powf(2.0);
    let real = (a.real * b.real + a.imaginary * b.imaginary)/quotient;
    let imaginary = (b.real * a.imaginary - a.real * b.imaginary)/quotient;
    cnum(real, imaginary)
}

fn cnum_to_string(c_num:&ComplexNumber) -> String{
    let sign = c_num.imaginary.signum();
    if sign == -1.0{
        format!("{}{}i", c_num.real, c_num.imaginary)
    }else{
        format!("{}+{}i", c_num.real, c_num.imaginary)
    }
}

fn modulus(c_num:&ComplexNumber) -> f32{
    (c_num.real.powf(2.0) + c_num.imaginary.powf(2.0)).sqrt()
}

fn conjugate(c:&ComplexNumber) -> ComplexNumber{
    cnum(c.real, -c.imaginary)
}
fn main() {
    let a = cnum(-2.0, 1.0);
    let b = cnum(1.0, 2.0);
    let c = cadd(&a, &b);
    let d = cmul(&a, &b);
    println!("{} PLUS {} Equals {}", cnum_to_string(&a), cnum_to_string(&b),  cnum_to_string(&c));
    println!("{} TIMES {} Equals {}", cnum_to_string(&a), cnum_to_string(&b),  cnum_to_string(&d));
    let c = modulus(&b);
    println!("The modulus of {} is {}", cnum_to_string(&b), c);
    println!("The conjugate of {} is {}", cnum_to_string(&a), cnum_to_string(&conjugate(&a)));
    println!("{} MINUS {} is {}", cnum_to_string(&a), cnum_to_string(&b), cnum_to_string(&csub(&a, &b)));
    println!("{} DIVIDED BY {} is {}", cnum_to_string(&a), cnum_to_string(&b), cnum_to_string(&cdiv(&a,&b)));
}
