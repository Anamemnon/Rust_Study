fn main() {
    let x = 5;
    let mut y = 3;
    print_number(x);
    print_number(y);
    println!("x + y = {}", sum(x, y));
    
    let f: fn(i32, i32) -> i32 = sum;
    println!("x + y = {}", f(x, y)); 

    y = 5;
    println!("x + y = {}", sum(x, y));
    
    
    y = diverges();
    println!("x + y = {}", sum(x, y));
}

fn print_number(x: i32) {
    println!("x = {}", x);
}

fn sum(x: i32, y: i32) -> i32 {
    if x == y {
        return x;
    }

    x + y
}

fn diverges() -> ! {
    panic!("panic in the fn 'diverges'");
}