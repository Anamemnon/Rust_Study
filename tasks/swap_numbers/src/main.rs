//! Даны две числовые переменные a, b.
//! Составьте фрагмент программы, после исполнения которого значения переменных
//! поменялись бы местами (новое значение a равно старому значению b и наоборот).
//! Решите задачу, не используя дополнительных переменных.


use std::ops::{Add, Sub};

fn main() {
    let mut a = 1;
    let mut b = 2;

    println!("a={} b={}", a, b);

    swap_numbers(&mut a, &mut b);

    println!("a={} b={}", a, b);
}

fn swap_numbers<T>(a: &mut T, b: &mut T)
    where T: Copy + Sub<Output = T> + Add<Output = T> {
    *a = *a + *b;
    *b = *a - *b;
    *a = *a - *b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_int_numbers() {
        let mut a = 1;
        let mut b = 2;

        swap_numbers(&mut a, &mut b);

        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }

    #[test]
    fn swap_double_numbers() {
        let mut a = 1.5;
        let mut b = 2.5;

        swap_numbers(&mut a, &mut b);

        assert_eq!(a, 2.5);
        assert_eq!(b, 1.5);
    }
}