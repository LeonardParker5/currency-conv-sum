use std::fs;

//const USD_TO_GBP: f64 = 0.71932;
const GBP_TO_USD: f64 = 1.38910;

fn main() {
    let contents = fs::read_to_string("file.txt")
        .expect("Something went wrong reading the file");

    let mut sum = 0.0;
    for word in contents.split_whitespace() {
        if let Some(pound_index) = word.find('£') {
            let chars: Vec<char> = word.chars().collect();
            if let Ok(num) = chars[pound_index+1..].iter().collect::<String>().parse::<f64>() {
                sum += num;
            }
        }
    }

    println!("Sum of pound amounts: £{:.2}", sum);

    let usd: f64 = sum * GBP_TO_USD;
    println!("${:.2} is the equivalent of £{:.2}", usd, sum);
}
