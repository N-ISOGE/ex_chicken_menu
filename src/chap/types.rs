extern crate rand;

pub fn give_num(one: i32, two: i32) -> i32 {
    // 지역변수 선언, 정의
    let result = one * two;
    // 함수 호출
    println!("wow {}", result);
    // code block 의 결과를 이용해 정의 가능, 편리한 문법
    let wow = { result * 10 };
    // 반환은 단순하게 반환할 것에 대한 표현식으로 작성 가능
    wow
}

//
//
//
//

#[test]
fn main() {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    println!("Hello, world! {}", give_num(2, 9));

    let secret_num = rand::thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse()//.expect("type number!");
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        let woo = 4;
        println!("Hello, world! {}", woo);
        {
            let wow2 = 10;
            println!("Hello, world! {}", wow2);
        }
        println!("Hello, world! {}", woo);
        println!("Hello! {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("right");
                break;
            }
        }
    }
}
