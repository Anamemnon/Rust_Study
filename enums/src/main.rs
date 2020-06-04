fn main() {
    let x: Message = Message::Move { x: 1, y: 2 };
    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };
    
    let n = 5;
    
    match n {
        1 => println!("1"),
        5 => println!("5"),
        _ => println!("?")
    }

    match x {
        Message::Move {x: 2, y: 3} => println!("2, 3"),
        Message::Move {x: 1, y: 2} => println!("1, 2"),
        _ => println!("?, ?")
    }
}

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

enum BoardGameTurn {
    Move { squares: i32 },
    Pass,
}
