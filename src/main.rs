use std::fs;

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
}
