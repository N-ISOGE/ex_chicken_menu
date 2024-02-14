//!
//! # 배경지식
//!
//! - type
//!     - [`various_scala_types`]
//!     - [`various_str_complex_types`]
//! - function, statements, expressions
//!     - [`function_name_is_snake_case`]
//!     - [`statements_expressions`]
//! - comment
//! - control flow
//!     - [`control_flow`]
//!
pub fn main() {
    println!("시작 -> 배경지식 the_backgroud_knowledge");
    // - [`type`]
    various_scala_types();
    various_str_complex_types();
    // - [`function, statements, expressions`]
    function_name_is_snake_case();
    statements_expressions();
    // - [`comment`]
    // - [`control flow`]
    control_flow();
    println!("배경지식 the_backgroud_knowledge -> 변수와 가변성 the_mutability_shadowing ")
}

/// ## Type
/// - 종류
///     - scala, 문자열, 복합
/// ### scala 타입
/// - 하나의 값으로 표현됨
/// - 정수형, 부동 소수점형, boolean, 문자형
/// - 정수형 부호표시 + 메모리 크기, \[iu\]크기
///     - i32가 주로 사용됨
///     - 리터럴로 표현할 때, 여러 진법을 이용할 수 있음
/// - 부동 소수점 f32, f64
///     - f64가 주로 사용됨
/// - 문자형 char, 작은 따옴표 사용. 문자열과 다름
fn various_scala_types() {
    let a_ingter: i64 = -123;
    println!("정수 i64 {} {:0x} ", a_ingter, 0xFFDF);

    let a_float: f64 = -123.21;
    println!("부동 소수점형 f64 {}", a_float);

    let a_char: char = '와';
    println!("문자형 char {}", a_char);
}

/// ### 문자열 타입
///
/// - &str
///     - 문자열을 참조
/// - String
///     - 표준 라이브러리에서 지원하는 타입
///
/// ### 복합 타입
///
/// - 다양한 값을 하나의 타입으로 묶음
/// - 튜플, 배열
/// - 튜플은 다양한 타입을 묶는 것, 배열을 같은 타입인 다양한 값을 묶는 것
///

fn various_str_complex_types() {
    let welcome_str: &str = "안녕하세요";
    let welcome_string /* :String */ = String::from("String");
    println!("{} {}", welcome_str, welcome_string);

    let a_tuple = (1, 2.0, "ddd");
    println!("{:?}", a_tuple);

    let a_array = [2, 3, 4, 5, 6];
    for number in a_array {
        println!("array {}", number)
    }
}

/// ## function, statements, expressions
///
/// - 구문은 어떤 명령들의 나열로 "값을 반환하지 않는" 어떤 동작을 수행 합니다.
/// - 표현식은 결과 값을 산출, 평가해냅니다.
/// - 블럭 {}은 구문, 표현식 다됨

/// ### function
/// - 변수, 함수 이름 스네이크 케이스 사용
fn function_name_is_snake_case() {
    println!("function_name_is_snake_case is function name")
}

/// ### statements, expressions
fn statements_expressions() {
    let y = {
        // 이건 값을 반환하지 않는 명령, 구문, statements
        let x = 3;
        // 이건 값을 반환하는 표현식, expressions인데
        // 이미 표현식의 블럭 안에 있는 것
        x + 1
    };

    println!("x를 만드는 구문으로 나온 x, x + 1를 처리하여 반환하는 표현식의 y : {y}");
}

/// ## comment
/// - 문서화 주석이라는 것이 있다, 문서화 주석에서 문서화 주석

/// ## control flow
/// - 구문, 표현식으로 둘 다 사용가능
/// - 반복문 loop, while, for
///
/// - 조건문에서 boolean 타입으로만 조건 검사 허용
///     - 다른 타입을 캐스팅해주지 않음
/// - loop 라벨 존재, 흐름을 정할 때 특정한 지점으로 보낼 수 있다.
fn control_flow() {
    let is_this_true = false;

    let a_never_mind = if is_this_true {
        "is_this_true is true"
    } else {
        "is_this_true is false"
    };

    let mut index = 0;
    loop {
        println! {"{} {}",a_never_mind, index};
        if index < 5 {
            index += 1;
        } else {
            break;
        }
    }
}
