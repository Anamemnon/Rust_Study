//! format!: записывает форматированный текст в String.
//! print!: работает аналогично с format!, но текст выводится в консоль (io::stdout).
//! println!: аналогично print!, но в конце добавляется переход на новую строку.
//! eprint!: аналогично format!, но текст выводится в стандартный поток ошибок (io::stderr).
//! eprintln!: аналогично eprint!, но в конце добавляется переход на новую строку.

fn main() {
    println!("Hello, world!");
    println!("{}, world!", "Hello");

    // Можно указать позицию для каждого аргумента.
    println!("{0}, это {1}. {1}, это {0}", "Алиса", "Боб");

    // Так же можно именовать аргументы.
    println!("{subject} {verb} {object}",
             object="ленивую собаку",
             subject="быстрая коричневая лиса",
             verb="прыгает через");
    
    // 1 из 10 людей знают, что такое двоичный код, а остальные нет.
    println!("{} из {:b} людей знают, что такое двоичный код, а остальные нет.", 1, 2);

    // Можно выравнивать текст, сдвигая его на указанную ширину.
    // Данный макрос отобразит в консоли
    // "     1". 5 пробелов и "1".
    println!("{number:>06.2}", number=1.256);
    println!("{number:>width$}", number=1, width=6);

    
    // Задания
    let pi = 3.141592;
    println!("pi = {0:.2}", pi)
}