fn main() {
    let x = 5;
    let mut y = 10;
    y += x;
    {
        let x = 25;
    }
    println!("x + y = {}", x + y);
}
