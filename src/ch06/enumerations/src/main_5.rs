#[derive(Debug)]
enum Messsage {
    Quit,
    Move { x: i32, y: i32 }, // 익명구조체
    Write(String), // (String) 값
    ChangeColor(i32, i32, i32), // (i32, i32, i32)
}

// struct QuitMessage; // 유닛구조체
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // 튜플구조체
// struct ChangeColorMessage(i32, i32, i32, i32); // 튜플구조체

fn main() {
    let quit = Messsage::Quit;
    let mov = Messsage::Move { x: 64, y: 128 };
    let write = Messsage::Write(String::from("Winter"));
    let chang_color = Messsage::ChangeColor(125, 125, 125);

    println!("{:#?}", quit);
    println!("{:#?}", mov);
    println!("{:#?}", write);
    println!("{:#?}", chang_color);
}
