fn main() {
    let mut x = 1;

    {
        let y = &mut x;
        *y += 1;
        let z = &mut x;
        *z += 2;
    }

    println!("{}", x);

}
