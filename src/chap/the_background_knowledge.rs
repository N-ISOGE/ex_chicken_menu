///
/// # 배경지식
///
pub fn main() {
    println!("시작 -> 배경지식 the_backgroud_knowledge");
    // - [`type`]
    various_types();
    // - [`function, statements, expressions`]
    function_name_is_snake_case();
    statements_expressions();
    // - [`comment`]
    // - [`control flow`]
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
fn various_types() {
    let a_ingter: i64 = -123;
    println!("정수 i64 {} {:0x} ", a_ingter, 0xFFDF);

    let a_float: f64 = -123.21;
    println!("부동 소수점형 f64 {}", a_float);

    let a_char: char = '와';
    println!("문자형 char {}", a_char);
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
/// - 구문, 표현식 둘 다 사용가능
/// - 반복문 loop, while, for
