// Импортируем (с помощью `use`) модуль `fmt`, чтобы мы могли его использовать.
use std::fmt::{self, Formatter, Display};

// Определяем структуру, для которой будет реализован `fmt::Display`.
// Это простая кортежная структура c именем `Structure`, которая хранит в себе `i32`.
struct Structure(i32);

// Чтобы была возможность использовать маркер `{}`
// `типаж (trait) fmt::Display` должен быть реализован вручную
// для данного типа.
impl fmt::Display for Structure {
    // Этот типаж требует реализацию метода `fmt` с данной сигнатурой:
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Записываем первый элемент в предоставленный выходной поток: `f`.
        // Возвращаем `fmt::Result`, который показывает выполнилась операция
        // успешно или нет. Обратите внимание на то, что синтаксис `write!`
        // похож на синтаксис `println!`.
        write!(f, "{}", self.0)
    }
}

// Определим структуру с именем `List`, которая хранит в себе `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Получаем значение с помощью индекса кортежа
        // и создаём ссылку на `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Пройдёмся по каждому `v` в `vec`.
        // Номер итерации хранится в `count`.
        for (count, v) in vec.iter().enumerate() {
            // Для каждого элемента, кроме первого, добавим запятую
            // до вызова `write!`. Используем оператор `?` или `try!`,
            // чтобы вернуться при наличие ошибок.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        // Закроем открытую скобку и вернём значение `fmt::Result`
        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
    }
}

fn main() {
    println!("{0}", Structure(10));

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Поменяйте {:?} на {}, когда добавите реализацию
        // типажа fmt::Display
        println!("{}", *color)
    }
}
