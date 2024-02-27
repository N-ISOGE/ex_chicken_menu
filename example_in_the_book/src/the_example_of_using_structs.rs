//!
//! # 구조체를 사용한 예시
//!
//! - 구조체를 사용할 코드
//!     - [`init_area_of_ellipse`]
//! - 튜플을 사용해보자
//!     - [`tuple_area_of_ellipse`]
//! - 구조체를 사용하여 리펙토링
//!     - [`struct_area_of_ellipse`]
//! - 트레이트 파생
//!     - [`trait_area_of_ellipse`]
//!

#[allow(unused)]
pub fn main() {
    println!("구조체란? the_defining_structs -> 구조체를 사용한 예시 the_example_of_using_structs");
    // - 구조체를 사용할 코드
    init_area_of_ellipse();
    // - 튜플을 사용해보자
    tuple_area_of_ellipse();
    // - 구조체를 사용하여 리펙토링
    struct_area_of_ellipse();
    // - 트레이트 파생
    trait_area_of_ellipse();
    println!("구조체를 사용한 예시 the_example_of_using_structs -> 메서드 문법 the_method_syntax");
}

///
/// ## 구조체를 사용할 코드
/// 예시로 타원의 넓이를 구하는 코드를 짜보자
///
/// - 하나의 타원에 대해서 2개의 매개변수를 받는다
/// - 매개변수간 관계가 애매해보일 수 있다.
///
/// -> 하나의 타원을 받게 해보자
///
fn init_area_of_ellipse() {
    fn area(a: f64, b: f64) -> f64 {
        a * b * std::f64::consts::PI
    }

    let a = 50.0;
    let b = 30.0;

    println!("기초적인 타원의 넓이: {:.4}", area(a, b));
}

///
/// ## 튜플을 사용해보자
///
/// 튜플을 매개변수로 받으면서 하나의 매개변수, 튜플로 받아서
/// 매개변수의 관계가 드러나보임.
/// 하지만 정확한 뜻이 모름.
/// 각각의 매개변수의 의미를 알리기 위해 구조체 사용
///
fn tuple_area_of_ellipse() {
    fn area(ellipse: (f64, f64)) -> f64 {
        ellipse.0 * ellipse.1 * std::f64::consts::PI
    }

    let a_ellipse = (50.0, 30.0);

    println!("튜플을 사용해 구한 타원의 넓이: {:.4}", area(a_ellipse));
}

///
/// ## 구조체를 사용하여 리펙토링
///
/// 타원 구조체를 정의해서 함수 내에서 명확한 의미가 표현됨.
///
fn struct_area_of_ellipse() {
    struct Ellipse {
        a: f64,
        b: f64,
    }

    fn area(object: &Ellipse) -> f64 {
        object.a * object.b * std::f64::consts::PI
    }

    let a_ellipse = Ellipse { a: 50.0, b: 30.0 };

    println!("구조체를 사용해 구한 타원의 넓이: {:.4}", area(&a_ellipse));
}

///
/// ## 트레이트 파생
///
///
/// `#[derive(Debug)]` 추가
/// - `Debug` 트레이트을 언어에서 구현하도록 외부 속성을 추가
///
fn trait_area_of_ellipse() {
    #[derive(Debug)]
    struct Ellipse {
        a: f64,
        b: f64,
    }

    fn area(object: &Ellipse) -> f64 {
        object.a * object.b * std::f64::consts::PI
    }

    let a_ellipse = Ellipse { a: 50.0, b: 30.0 };

    println!("튜플을 사용해 구한 타원의 넓이: {:.4}", area(&a_ellipse));
    println!("출력 방법을 구현된 타원 : {:?}", a_ellipse);
}
