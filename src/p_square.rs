use crate::convert::string_to_int;
use crate::square_root::sqrts;

pub fn perfectsquare(num_str: &str) -> (usize, Vec<f64>) {
    let mut num = string_to_int(num_str) as f64;
    let mut result = num / 2.0;
    let mut count = 0;
    let mut perfect_squares = Vec::new();

    if (num - (result * result)).abs() < f64::EPSILON {
        let result2 = sqrts(result).floor();
        if (result2 - result2.floor()).abs() < f64::EPSILON {
            count = 1;
            perfect_squares.push(result2);
            return (count, perfect_squares);
        }
    } else {
        result = sqrts(num).floor();
        let mut result2 = num - (result * result);
        while result2.abs() > f64::EPSILON {
            result = sqrts(num).floor();
            let square = result * result;
            num -= square;
            result2 = num;
            count += 1;
            perfect_squares.push(result);
        }
    }

    (count, perfect_squares)
}
