

pub fn sqrts(x: f64) -> f64 {
    if x < 0.0 {
        panic!("Cannot calculate square root of negative number");
    }

    if x == 0.0 || x == 1.0 {
        return x;
    }

    let mut guess = x / 2.0;
    let  precision = 0.00001;

    while (guess * guess - x).abs() > precision {
        guess = (guess + x / guess) / 2.0;
    }

    guess
}

//fn main() {
//    let x = 16.0;
//    let sqrt_x = sqrt(x);
//    println!("The square root of {} is {}", x, sqrt_x);
//}