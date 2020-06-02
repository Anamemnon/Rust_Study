fn main() {
    let mut a = 1;
    loop {
        println!("loop: Hello, world!");
        a +=1;
        if a == 5 {
            break;
        }
    }

    a = 1;
    while a != 5 {
        println!("while: a = {}", a);
        a += 1;
    }

    for i in 10..15 {
        println!("for: i = {}", i);
    }
    
    for (index, value) in (5..10).enumerate() {
        println!("index {}, value {}", index, value);
    }

    let lines = "привет\nмир\nhello\nworld".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    
    'outer: for x in 0..6 {
        'inner: for y in 0..6 {
            if x % 2 == 0 { continue 'outer; } // продолжает цикл по x
            if y % 2 == 0 { continue 'inner; } // продолжает цикл по y
            println!("x: {}, y: {}", x, y);
        }
    }
}
