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

fn imul(a:&ComplexNumber, b:&ComplexNumber) -> ComplexNumber{
    let real = a.real * b.real + a.imaginary * b.imaginary * -1.0;
    let imaginary = a.real * b.imaginary + a.imaginary * b.real;
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

fn main() {
    let a = cnum(3.0, -1.0);
    let b = cnum(1.0, 4.0);
    let c = cadd(&a, &b);
    let d = imul(&a, &b);
    println!("{} PLUS {} Equals {}", cnum_to_string(&a), cnum_to_string(&b),  cnum_to_string(&c));
    println!("{} TIMES {} Equals {}", cnum_to_string(&a), cnum_to_string(&b),  cnum_to_string(&d));

}
