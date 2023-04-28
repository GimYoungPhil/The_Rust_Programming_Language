use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 맞혀봅시다!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("사용자가 맞쳐야 할 숫자: {}", secret_number);

    loop {
        println!("정답이라고 생각하는 숫자를 입력하세요!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("입력한 값: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            },
        }
    }
}


fn test() {
    let a: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4];
    let b: [f32; 4] = [0.5, 0.6, 0.7, 0.8];
  
    let c: &[f32] = &a;
    let d: &[f32; 4] = &b;

    println!("{:?}", c);
    println!("{:?}", d);
  }
  