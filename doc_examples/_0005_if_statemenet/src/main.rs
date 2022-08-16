fn main() {
    // 1
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 2
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

    // 3
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
