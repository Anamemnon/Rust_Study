use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input = input.trim()
                     .split_whitespace()
                     .map(|x| x.parse::<u64>().unwrap() as u64)
                     .collect::<Vec<_>>();
    
    let m = (input[0] as f64 / input[2] as f64).ceil() as u64;
    let n = (input[1] as f64 / input[2] as f64).ceil() as u64;


    println!("{}", m * n);
}
