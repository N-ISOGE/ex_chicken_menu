//! ## match 표현식
//!
//!

pub fn main() {
    println!("{} wow", value_in_cents(Coin::random_coin()));
    println!("{} wow us coin", value_in_us_cents(USCoin::random_coin()));
}

/// ## `match`, 제어 흐름 연산자
/// 일련의 패턴들에 따라 값을 비교한 뒤, 어떤 패턴에 매칭되었는지를 바탕으로 코드를 수행하도록 해줌.
/// - 패턴은 리터럴 값, 변수명, 와일드카드등 여러 방법이 존재, 18장에서 상세하게 나올 예정
///
/// 패턴을 사용과 더불어 컴파일러가 가능한 모든 경우가 처리되는 지 검사함.
///
#[allow(dead_code)]
struct OverviewMatchExpression;

/// 예시용 동전
#[derive(Copy, Clone, PartialEq)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    ///
    ///
    /// 추상화된 함수가 있나?
    /// fn random_init<enum_like T>() -> Self {
    ///     T::members.choose(&mut thread_rng).unwrap()
    /// }
    ///
    fn random_coin() -> Self {
        use rand::prelude::*;
        match thread_rng().gen_range(0..4) {
            0 => Coin::Penny,
            1 => Coin::Nickel,
            2 => Coin::Dime,
            3 => Coin::Quarter,
            _ => panic!("out of range!"),
        }
    }
}

/// ### 동전을 이용한 예시
/// 동전 받아서 바꾸기
///
/// `match`의 특징
/// - `match` 뒤에 표현식따라 옴. 어떤 값이 오든 상관없음.
/// - 내부에서는 패턴과 코드로 이루어진 갈래가 존재.
///     - 각 갈래는 쉼표로 구분
/// - 순차적으로 패턴과 비교하다가 값이 패턴과 매칭되면 코드를 실행함.
///     - 코드를 코드 블럭으로 변경 가능
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("니켈 ㅇㅇ");
            5
        }
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/// 예시용 미국 주들
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn random_state() -> Self {
        use rand::prelude::*;
        match thread_rng().gen_range(0..2) {
            0 => UsState::Alabama,
            1 => UsState::Alaska,
            _ => panic!("out of range!"),
        }
    }
}

/// 예시용 미국 동전
enum USCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl USCoin {
    fn random_coin() -> Self {
        use rand::prelude::*;
        match thread_rng().gen_range(0..4) {
            0 => USCoin::Penny,
            1 => USCoin::Nickel,
            2 => USCoin::Dime,
            3 => USCoin::Quarter(UsState::random_state()),
            _ => panic!("out of range!"),
        }
    }
}

/// ### 값을 바인딩 하는 예시
///
fn value_in_us_cents(us_coin: USCoin) -> u8 {
    match us_coin {
        USCoin::Penny => 1,
        USCoin::Nickel => 5,
        USCoin::Dime => 10,
        USCoin::Quarter(state) => {
            println!("what states make this? {:?}", state);
            25
        }
    }
}
