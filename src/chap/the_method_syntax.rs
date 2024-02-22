//!
//! # 메서드 문법
//!
//! - 메서드?
//! - 메서드 정의하는 예시
//!     - [`method_area_of_ellipse`]
//! - 멤버 접근 연산자 중 -> 연산자가 없음
//!     - [`auto_refer_and_derefer`]
//! - 매개변수가 늘어난 메서드
//!     - [`method_with_parameters`]
//! - 연관 함수
//!     - [`associated_func`]
//!

#[allow(unused)]
pub fn main() {
    println!("구조체를 사용한 예시 the_example_of_using_structs -> 메서드 문법 the_method_syntax");

    // - 메서드?
    // - 메서드 정의하는 예시
    method_area_of_ellipse();
    // - 멤버 접근 연산자 중 -> 연산자가 없음
    auto_refer_and_derefer();
    // - 매개변수가 늘어난 메서드
    method_with_parameters();
    // - 연관 함수
    associated_func();
    println!("메서드 문법 the_method_syntax -> 열거형 정의하기 the_defining_an_enum");
}

#[derive(Debug)]
struct Ellipse {
    a: f64,
    b: f64,
}

impl Ellipse {
    fn area(&self) -> f64 {
        self.a * self.b * std::f64::consts::PI
    }
    // fn a(&self) -> f64 {
    //     self.a
    // }
}

///
/// ## 메서드?
///
/// 함수랑 비슷함 대신 차이점이 있다.
/// - 구조체 컨텍스트에 정의되는 점
/// - 첫 번째 매개변수가 항상 `self`라는 점
///     - 여기서 `self`는 구조체 인스턴스를 가리킴
///
/// ## 메서드 정의하는 예시
///
fn method_area_of_ellipse() {
    #[derive(Debug)]
    struct Ellipse {
        a: f64,
        b: f64,
    }

    impl Ellipse {
        fn area(&self) -> f64 {
            self.a * self.b * std::f64::consts::PI
        }
        fn a(&self) -> f64 {
            self.a
        }
    }
    let a_ellipse = Ellipse { a: 50.0, b: 30.0 };

    println!("getter 구현된 타원 : {:?}", a_ellipse.a());
    println!("역참조 구한 타원의 넓이: {:.4}", a_ellipse.area());
    println!("출력 방법을 구현된 타원 : {:?}", a_ellipse);
}
///
/// ## 멤버 접근 연산자 중 -> 연산자가 없음
///
/// 러스트에서는 자동 참조 및 역참조로 . 연산자를 쓰면 메서드에 맞체 &, &mut, *을 추가함.
///

fn auto_refer_and_derefer() {
    let a_ellipse = Ellipse { a: 50.0, b: 30.0 };

    // 동일한 연산
    #[allow(clippy::needless_borrow)]
    println!("타원의 넓이: {:.4}", (&a_ellipse).area());
    println!(" 넓이 출력 : {:.4}", a_ellipse.area());
}

///
/// ## 매개변수가 늘어난 메서드
///
/// 함수의 서명, 매개변수의 타입을 판단함
/// -   매개변수로 받은 값을 변형할 필요없으니 불변, 소유권이 필요없으니 참조자
///
fn method_with_parameters() {
    impl Ellipse {
        fn can_hold(&self, other: &Ellipse) -> bool {
            let my_long = if self.a > self.b { self.a } else { self.b };
            let my_short = if self.a < self.b { self.a } else { self.b };
            let other_long = if other.a > other.b { other.a } else { other.b };
            let other_short = if other.a < other.b { other.a } else { other.b };

            my_long >= other_long && my_short >= other_short
        }
    }

    let ellipse1 = Ellipse { a: 40.0, b: 30.0 };
    let ellipse2 = Ellipse { a: 30.0, b: 20.0 };
    let ellipse3 = Ellipse { a: 20.0, b: 60.0 };

    println!(
        "타원 1에 타원 2가 들어가나? {}",
        ellipse1.can_hold(&ellipse2)
    );
    println!(
        "타원 1에 타원 3이 들어가나? {}",
        ellipse1.can_hold(&ellipse3)
    );
}
///
/// ## 연관 함수
///
/// impl 블록내에 구현된 모든 함수를 연관 함수라고 부름.
/// -   impl 뒤에 나오는 타입과 연관된 함수들
///
/// 이중 self를 첫 매개변수가 아닌 연관함수도 존재
/// - 예시 String::from(), :: 연산자로 접근, 생성자로 주로 사용
///
/// 구조체에 대한 impl 블록은 여러 개 존재 가능
///
fn associated_func() {
    impl Ellipse {
        fn circle(diameter: f64) -> Self {
            Self {
                a: diameter,
                b: diameter,
            }
        }
    }

    let circle = Ellipse::circle(50.0);

    println!("(타)원 {:?}", circle);
}
