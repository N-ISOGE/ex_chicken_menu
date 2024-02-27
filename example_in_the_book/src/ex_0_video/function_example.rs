///
/// 1. 함수
///
/// - 블럭 안에서 함수 정의해서 함수를 쉐도잉, 지역을 우선함
/// - 컴퓨터가 대신 여러번 출력해줌
/// - 임의의 곳에서 수정해도 여러곳에 적용
/// - 동일 패키지에서 함수명 중복 불가
/// - 도구를 이용하기, 원하는 데이터를 받는 표현식을 쓰고 도구가 어떻게 평가하는지 보고선 타입 적기
///
#[allow(dead_code)]
pub fn main() {
    println!("컴퓨터가 대신 여러번 출력해줌");
    println!("임의의 곳에서 수정해도 여러곳에 적용");
    println!("동일 패키지에서 함수명 중복 불가");
    range_of_function_definition();
    println!(r#"
    도구를 이용하기, 원하는 데이터를 받는 표현식을 쓰고 도구가 어떻게 평가하는지 보고선 타입 적기
    근데 추상적이여야 하는 상황이 존재하네...
    "#);
    println!(r#"
    스택과 힙, Copy 트레이트 햇갈린다.
    참조자가 아니면 원격으로 수정할 수 없다.
    "#);
}

fn range_of_function_definition() {
    println!("블럭 안에서 함수 정의해서 함수를 쉐도잉, 지역을 우선함");
    fn f1() -> String { String::from("외부") }
    {
        println!("{}", f1());
        fn f1() -> String { String::from("내부 1") }
    }
    println!("{}", f1());
    {
        fn f1() -> String { String::from("내부 2") }
        println!("{}", f1());
    }
}
