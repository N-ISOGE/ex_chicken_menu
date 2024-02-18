//!
//! # 메서드 문법
//! 

#[allow(unused)]
pub fn main(){
    println!("구조체를 사용한 예시 the_example_of_using_structs -> 메서드 문법 the_method_syntax");

    method_area_of_ellipse();
    println!("메서드 문법 the_method_syntax -> 열거형 정의하기 the_defining_an_enum");
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

    impl Ellipse{
        fn area(&self) -> f64 {
            self.a * self.b * std::f64::consts::PI
        }
    }

    let a_ellipse = Ellipse { a: 50.0, b: 30.0 };

    println!("역참조 구한 타원의 넓이: {:.4}", a_ellipse.area());
    println!("출력 방법을 구현된 타원 : {:?}",a_ellipse);
}