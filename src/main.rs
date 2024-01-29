mod chap;
mod example;
#[path = "치킨메뉴/mod.rs"]
mod 치킨메뉴;

fn calc_len(s: &String) -> usize {
    // &String : String에 대한 참조자
    s.len()
}

fn main() {
    println!(" {} ", chap::types::give_num(10, 30));
    let s1 = String::from("hello");
    async {
        let end =  example::main();
        calc_len(&s1); // & 연산자 사용, 참조자
        println!("{} {}", s1, end.await.is_ok());
    };
}
