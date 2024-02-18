//!
//! # 참조와 대여
//! 
//! ## 요약
//! - 스코프에서 가변 참조자 하나, 혹은 여러 불변 참조자 유효 가능
//! - 참조자는 항상 유효해야 함? 가르키는 대상이 유효?
//! 
//! ## 개요
//! - 참조자
//!     - [`reference_example`]
//! - 가변 참조자
//!     - [`mutable_reference_example`]
//!     - [`range_of_reference`]
//! - 댕글링 참조
//!     - [`make_dangling_reference`]
//!
#[allow(unused)]
pub fn main() {
    println!("소유권 the_ownership -> 참조와 대여 the_refer_borrow");
    // - 참조자
    reference_example();
    // - 가변 참조자
    mutable_reference_example();
    range_of_reference();
    // - 댕글링 참조
    make_dangling_reference();
    println!("참조와 대여 the_refer_borrow -> 슬라이스 the_slice");
}

/// ## 참조자
///
/// 요약
/// - 스코프에서 가변 참조자 하나, 혹은 여러 불변 참조자 유효 가능
/// - 참조자는 항상 유효해야 함? 가르키는 대상이 유효?
/// 
/// 함수의 매개변수로 변수를 넘기고 반환값으로 변수를 받는 대신,
/// **값의 소유권을 넘기는 대신**
/// 참조자를 전달할 수 있다.
///
/// 참조자는 포인터와 같은 것.
/// - 다른 변수가 소유하고 있는 값에 접근 가능
/// - 살아있는 동안 특정 타입에 대한 유효한 값을 가리킴을 보장
///
/// 참조자 사용 예시
///
fn reference_example() {
    let s1 = String::from("안녕하세요");

    // s1 대신 &s1 사용
    let len = calculate_length(&s1);

    println!(" '{}'의 길이 : {}.", s1, len);
}

/// 여기서 s는 참조자로 스코프 밖으로 벗어나도 참조자가 가리키는 대상은
/// 버려지지 않는다.
/// 이렇게 사용하는 것을 대여라고 함.
// String 대신 &String 사용
fn calculate_length(s: &str) -> usize {
    // 참조자도 기본적으로 불변성을 띔
    // s.push_str("!");
    s.len()
}

///
/// ## 가변 참조자
///
/// `mut` 키워드를 추가하면 참조자가 가리키는 값을 변경할 수 있다.
/// 제약사항으로 어떤 값에 대한 가변 참조자가 있으면 참조자를 생성할 수 없다.
/// - 제한사항으로 특정한 스코프 내 특정한 데이터 조각에 대한 가변 참조자를 `하나만` 만들 수 있다
///
fn mutable_reference_example() {
    let mut s = String::from("바꾸기");
    println!("{}", s);

    // 불변 참조자가 있으면 가변 참조자 생성 불가
    // 불변할 것이라는 예상을 보장
    // let r0 = & s;
    let r1 = &mut s;
    // mut 상관없이 참조자 생성 불가
    // let r2 = & s;

    change_mut_ref(r1);
    println!("{}", s);
}

fn change_mut_ref(s: &mut String) {
    s.push_str(" 가능");
}

/// 같은 데이터에 대해 여러 가변 참조자 사용을 막는 제약
/// - 제약이 제어를 쉽게 하기 위한 유도책
/// - data race 방지
///
/// 참조자가 유효한 범위
fn range_of_reference() {
    let mut s = String::from("범위");

    // 이동한 뒤 참조자가 유효하지 않게 됨
    let r0 = &s;
    let r1 = &s;
    println!("{} {}", r0, r1);

    // 앞의 참조자들이 유효하지 않음으로 사용 가능
    let r2 = &mut s;
    println!("{} 끝남", r2);
}

/// ## 댕글링 참조
/// 
/// 다른 곳에서 해제가 됐는지 모르는 포인터
/// 러스트에서는 참조자를 만드면 해당 참조자가 스코프를 벗어나기 전에
/// 데이터가 스코프를 벗어나는지 확인하여 댕글링 참조가 생성되는 지 검사함
/// 
/// 댕글링 참조 예시
/// dangle 반환값이 &String이면 다음과 같은 에러가 뜸
/// 
/// > missing lifetime specifier  
/// > this function's return type contains a borrowed value,  
/// > but there is no value for it to be borrowed  
/// >  from rustc
/// - 빌린 것을 반환 하는데 반환 받은 빌린 것에 값이 없음
/// 
fn make_dangling_reference() {
    let reference_to_nothing = dangle();
    println!("{}",reference_to_nothing)
}

#[allow(clippy::let_and_return)]
fn dangle() -> /*&*/String {
    let s = String::from("댕글링이면 안 나옴");
    /*&*/s
} // 여기서 이동하지 않으면 s가 소멸함, &s는 소멸한 값 가르킴
 