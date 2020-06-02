fn main() {
    let x       = true;
    let y: bool = false;

    println!("x = {}, y = {}", x, y);

    let x = 'x';
    let y = '💕';
    println!("x = {}, y = {}", x, y);

    let i8_:    i8    = -127;
    let u8_:    u8    =  255;
    let i16_:   i16   = -32768;
    let isize_: isize =  9223372036854775807;
    let usize_: usize =  18446744073709551615;

    println!("i8 {}",    i8_);
    println!("u8 {}",    u8_);
    println!("i16 {}",   i16_);
    println!("isize {}", isize_);
    println!("usize {}", usize_);

    let     x = [1, 2, 3]; // x: [i32; 3]
    //let mut y = [4, 5, 6]; // y: [i32; 3]
    let     a = [0; 20];   // a: [i32; 20]

    println!("x.len() = {}", x.len());
    //println!("y.len() = {}", y.len());
    println!("a.len() = {}", a.len());

    println!("x[1] = {}", x[1]);

    //let slice_x = &x[..];
    //let slice_x_end = &x[1..3];

    let x: (i32, &str) = (1, "привет"); // tuple
    println!("x.1 = {}", x.1);
}
