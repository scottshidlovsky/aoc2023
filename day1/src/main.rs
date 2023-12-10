use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should be able to read this input file");
    let mut overall_score = 0;
    for line in contents.split_whitespace() {

        let mut first_digit: Option<char> = Option::None;
        let mut last_digit: Option<char> = Option::None;
        for char in line.chars() {
            if char.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(char)
                }
                last_digit = Some(char)
            }
        }
        let mut u = first_digit.unwrap().to_string() ;
        u.push(last_digit.unwrap());
        overall_score += u.parse::<i32>().unwrap();
    }
    println!("Overall Score:\n{overall_score}")
}
