//! # 초보일 때 할만한 실수
//!
//! [참조](https://www.youtube.com/watch?v=PbR4ECFIckg)
//!
//! - 필요한 참조형식으로 쓰기
//! - 슬라이스 인덱싱을 적당하게 사용하기
//! - sentinel value, 초계값, 끝값?
//! - Enum 쓰기
//! - 에러 핸들링 하기
//! - standard lib 트레이트 사용
//! - standard lib 메크로 활용하기
//! - 도구 활용하기
//! - 코드 구조
//!

#[allow(unused)]
pub fn main() {
    todo_organizing();
    // - 필요한 참조형식으로 쓰기
    unnecessary_indirection();
}

///
/// ## 필요한 참조형식으로 쓰기
/// 
/// > &str로 넘겨서 사용할 수 있는 &String 매개변수라면
/// > &str으로 매개변수의 타입을 선언하자.
/// > 다른 타입도 유연하게 받을 수 있는 함수가 된다.
/// 
fn unnecessary_indirection() {
    fn before_print_fn(s:&String){
        println!("~ s");
        println!("{s}");
        println!("! s");
    }
    fn after_print_fn(s:&str){
        println!("~ s");
        println!("{s}");
        println!("! s");
    }

    let a_string = String::from("String 객체");
    let a_str = "문자열 리터럴, 문자열 불변 참조자";
    before_print_fn(&a_string);
    // 받을 수 있는 형식이 제한됨.
    // before_print_fn(a_str);
    after_print_fn(&a_string);
    after_print_fn(a_str);
}



/// ## 슬라이스 인덱싱을 적당하게 사용하기
///
/// 쉽지만 쓰다가 문제생김 (범위를 벗어난 접근)
/// map으로 보내서 알아서 범위를 정하게 함
///
/// ## sentinel value, 초계값, 끝값?
///
/// 함수 끝날 때, 특정한 조건에서 특정한 값을 쓰는 일이 있다.
/// - 빈 값을 반환하는 경우에 "", -1, null
/// 그 대신에 `Option<>`을 사용
///
/// ## Enum 쓰기
///
/// 쓰면 명확해짐
///
/// 패턴 매칭?
/// - 어떤 조건을 가정하기 위해 코드를 여러개 쓰지 않고 한번에 처리
/// - 조건문에 패턴을 넣어 확인?
///
/// ## 에러 핸들링 하기
///
/// 물음표 연산자로 에러 확인하기
/// 개인적으로 만든 에러 타입을 구현 하기
/// - 메크로를 이용
///
/// ## standard lib 트레이트 사용
///
/// From, TryFrom
/// FromStr
///
/// ## standard lib 메크로 활용하기
///
/// todo!()
/// concat, format
///
/// ## 도구 활용하기
/// - cargo fmt
/// - cargo-clippy
///
/// ## 코드 구조
/// Rc, Arc
///
fn todo_organizing() {
    todo!("정리 할 것");
}
