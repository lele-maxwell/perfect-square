pub fn string_to_int(s: &str) -> f64 {
    let mut result  = 0.0;
    for c in s.chars() {
        if let Some(digit) = c.to_digit(10) {
            result = result * 10.0 + digit as f64;
        } else {
            panic!("Invalid character in input");
        }
    }
    result
}

//fn main() {
//let input = "123";
//    let output = string_to_int(input);
//    println!("Output: {}", output);
//}