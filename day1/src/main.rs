use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should be able to read this input file");
    let mut overall_score = 0;
    let digit_words = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    for line in contents.split_whitespace() {

        let mut first_digit: Option<char> = Option::None;
        let mut last_digit: Option<char> = Option::None;
        for (index, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(char)
                }
                last_digit = Some(char)
            } else {
                let (_, last) = line.split_at(index);

                match digit_words.iter().find(|(x, _)| {
                    return last.starts_with(*x);
                }) {
                    Some((_, value)) => {
                        if first_digit.is_none() {
                            first_digit = Some(*value)
                        }
                        last_digit =  Some(*value)
                    }
                    _ => (),
                }
            }
        }
        // let f = first_digit.unwrap();
        // let f1 = last_digit.unwrap();
        // println!("{line} {f} {f1}");
        let mut u = first_digit.unwrap().to_string() ;
        u.push(last_digit.unwrap());
        overall_score += u.parse::<i32>().unwrap();
    }
    println!("Overall Score:\n{overall_score}")
}
