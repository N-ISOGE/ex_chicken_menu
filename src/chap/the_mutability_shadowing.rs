///
/// # 변수와 가변성
///
/// - [`러스트가 의도한 점`]
/// - [`러스트에서 변수`]
/// - [`상수`]
/// - [`섀도잉`]
///
#[allow(unused)]
pub fn main() {
    println! {"배경지식 the_backgroud_knowledge -> 변수와 가변성 the_mutability_shadowing"}
    // - [`러스트가 의도한 점`]
    // - [`러스트에서 변수`]
    immutable_variable();
    mutable_variable();
    // - 상수
    using_constant();
    // - [`섀도잉`]
    example_of_shadowing();
    same_name_but_different_type();

    println! {"변수와 가변성 the_mutability_shadowing -> 소유권 the_ownership"};
}

/// ## 러스트가 의도한 점
/// - 러스트에서 변수는 기본적으로 불변임
///     - 안정성과 쉬운 동시성을 취하도록 유도
/// - 가변으로 만들 수 있음, 다음과 같은 설명이 있음
///     - 가변으로 바꾸는 방법
///     - 불변성을 권하는 이유
///     - 가변성을 사용하는 상황
///

/// ## 러스트에서 변수
/// ### 불변성
/// 변수가 불변일 때, 값을 변경하려 시도하는 예시
fn immutable_variable() {
    let immutable = 5;
    println!("immutable 값 : {}", immutable);
    // 아래 줄을 주석해제하면 에러 발생
    // immutable = 6;
    println!("immutable 값 : {}", immutable);
}

/// 컴파일러가 오류를 출력함
///
/// error[E0384]: cannot assign twice to immutable variable `immutable`  
/// --> src/chap/the_mutability_shadowing.rs:29:5  
///    |  
/// 26 |     let immutable = 5;  
///    |         ---------  
///    |         |  
///    |         first assignment to `immutable`  
///    |         help: consider making this binding mutable: `mut immutable`  
/// ...  
/// 29 |     immutable = 6;  
///    |     ^^^^^^^^^^^^^ cannot assign twice to immutable variable  
///  
/// For more information about this error, try `rustc --explain E0384`.  
///  
/// cannot assign twice to immutable variable `immutable`
/// -> 불변 변수 x에 두번 할당 불가
/// - 컴파일러가 경고를 해줌
///
/// 버그가 생길 수 있는 상황을 컴파일 타임에 검사하는 것으로 방지됨.
/// 변수가 변하여 문제가 생기는 상황
/// - 변수를 불변으로 설정하면 러스트 컴파일러는 이를 보증함.
/// - 변수의 값이 어디서 어떻게 변하는지 추적할 필요 없음.
///
/// ### 가변성
/// 변수명 앞에 `mut`을 붙여서 가변으로 만들 수 있다.
/// 적절한 상황에서 적절한 선택으로 적절하게 사용하자.
fn mutable_variable() {
    let mut mutable = 5;
    println!("mutable 값 : {}", mutable);
    mutable = 6;
    println!("mutable 값 : {}", mutable);
}

/// ## 상수, constant
///
/// 불변 변수처럼 불변하지만 다른 점이 있다.
/// - `mut` 사용 불가, 애초에 개념에서부터 불변
/// - `const`를 사용하여 선언, `let` 아님
/// - 선언시 타입을 명시해야 함.
/// - 상수 표현식으로만 설정해야함, 컴파일시 결정되는 값을 사용.
///
/// 상수 이름은 러스트 이름 짓기 관례대로 대문자와 언더바를 사용한다.
fn example_of_constant() -> i64 {
    const MAX_LEVEL: i64 = max_level() + 7;
    MAX_LEVEL
}
fn using_constant() {
    println!("wow {} level so much {}", MAX_LEVEL, example_of_constant());
}
/// 전역으로 설정, 컴파일 타임에 결정되는 표현식이 쓰임
/// 상수가 선언된 스코프 내에서, 프로그램이 동작하는 전체 시간동안 유효합니다.
const MAX_LEVEL: i64 = max_level();
const fn max_level() -> i64 {
    43
}
/// 하드코딩한 값을 상수로 두면 나중에 바꾸기 편하다.

/// # 섀도잉
///
/// 새 변수를 이전 변수명과 같은 이름으로 선언하는 것을 섀도잉이라 부름
/// 이전에 선언했던 변수를 가려버림
/// - 스스로를 가려버림
/// - 변수 이름을 사용해도 대신 사용됨.
///
/// 이전의 변수에 접근이 불가능함이 안전하다?
/// - 변수가 언제 변경되는지 제어하기가 쉽다
/// - imutable 변수라면 다시 선언하기 전까지 값이 바뀌지 않는다.
/// - 선언하는 지점만 고려하면 됨.
/// - 선언하고 정의한 값이 내가 얻는 결과값임.
///
/// `let` 키워드, 이전 변수명을 사용해서 섀도잉을 할 수 있다.
fn example_of_shadowing() {
    let x = 10;
    // 섀도잉 함.
    let x = x + 1;
    {
        // 섀도잉 함.
        let x = x * 2;
        println!("이 안 스코프에서는 x 값은 {}", x);
    }
    println!("이 밖 스코프에서는 x 값은 {}", x);
}
/// 섀도잉은 변수를 `mut`로 표시하는 것과 다르다.
/// `let` 키워드를 통해 새로운 변수를 만드는 것
/// - 타입을 다르게 해서 만들 수 있다.
fn same_name_but_different_type() {
    let wowow = "wowow";
    let wowow = wowow.len();
    println!("wowow 값 : {}", wowow);
    // 아래는 에러가 뜸
    // let mut wowow = "wowow";
    // wowow = wowow.len();
    // println!("wowow 값 : {}", wowow);
}
