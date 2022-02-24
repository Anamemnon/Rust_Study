fn main() {
    let mut point = Point {x: 1, y: 2};

    {
        let point_ref = PointRef{x: &mut point.x, y: &mut point.y};
        *point_ref.x = 5;
        *point_ref.y = 10;
    }

    println!("x = {}, y = {}", point.x, point.y);

    let black = Color(10, 0, 0);
    println!("black.0 = {}", black.0);

    struct Inches(i32);
    let length = Inches(3);
    let Inches(int_len) = length;

    println!("int_len = {}", int_len)
}

struct Point {
    x: i32,
    y: i32,
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

struct Color(i32, i32, i32);