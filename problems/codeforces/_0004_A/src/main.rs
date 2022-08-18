use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    
    let input: u8 = input.trim()
        .parse()
        .expect("Error parsing input");
    
    if input > 3 && input % 2 == 0 { 
        println!("YES");
    } else {
        println!("NO");
    }
}
