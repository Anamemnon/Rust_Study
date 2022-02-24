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

    // Целочисленное сложение
    println!("1 + 2 = {}", 1u32 + 2);

    // Целочисленное вычитание
    println!("1 - 2 = {}", 1i32 - 2);
    // ЗАДАНИЕ ^ Попробуйте изменить `1i32` на `1u32`
    // чтобы убедится насколько важен тип данных

    // Булева логика
    println!("true И false будет {}", true && false);
    println!("true ИЛИ false будет {}", true || false);
    println!("НЕ true будет {}", !true);

    // Побитовые операции
    println!("0011 И 0101 будет {:04b}", 0b0011u32 & 0b0101);
    println!("0011 ИЛИ 0101 будет {:04b}", 0b0011u32 | 0b0101);
    println!("0011 исключающее ИЛИ 0101 будет {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 будет {}", 1u32 << 5);
    println!("0x80 >> 2 будет 0x{:x}", 0x80u32 >> 2);

    // Использование подчёркивания для улучшения читаемости!
    println!("Один миллион записан как {}", 1_000_000u32);
}
