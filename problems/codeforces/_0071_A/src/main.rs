use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading input");

    let n: u8 = n.trim().parse().expect("");

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if is_too_long(&input) {
            println!("{}{}{}", input.chars().nth(0).unwrap(), input.len() - 2, input.chars().last().unwrap());
        } else {
            println!("{input}");
        }
    }
}

fn is_too_long(s: &str) -> bool {
    s.len() > 10
}
