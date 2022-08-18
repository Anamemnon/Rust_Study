use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading input");
    let n: u16 = n.trim().parse().expect("Error parsing");

    let mut counter: u16 = 0;

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");
        let array: Vec<u8> = input.trim()
                                  .split_whitespace()
                                  .by_ref()
                                  .map(|x| x.parse::<u8>().unwrap())
                                  .collect();
        if array.iter().sum::<u8>() >= 2 {
            counter += 1;
        }
    }

    println!("{counter}");
}
