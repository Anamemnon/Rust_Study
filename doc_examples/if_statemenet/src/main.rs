fn main() {
    let mut a = 1;
    let mut b = 2;
    let x = if a == b {
        a += b;
        a
    } else {
        b *= 2;
        b
    };

    println!("x = {}", x);
}
