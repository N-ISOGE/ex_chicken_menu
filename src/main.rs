mod chap_ownership;
#[path = "치킨메뉴/mod.rs"]
mod 치킨메뉴;

fn main() {
    println!("ownership 실행");

    let end = example::main();

    println!("잘 됨? {}", end.is_ok());
}
