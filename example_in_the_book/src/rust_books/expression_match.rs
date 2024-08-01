//! ## match 표현식
//!
//! - [`OverviewMatchExpression`]
//! - [`value_in_cents`]
//! - [`value_in_us_cents`]
//! - [`plus_one`]
//! - [`ExhaustiveMatch`]
//! - [`catch_all_and_under_bar`]
//! - [`other_if_let`]
//!

pub fn main() {
    println!("{} wow", value_in_cents(Coin::random_coin()));
    println!("{} wow us coin", value_in_us_cents(USCoin::random_coin()));
    print_sample_of_plus_one();
    catch_all_and_under_bar();
    other_if_let();
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

/// ### 매칭된 결과 중 일부분을 바인딩 하는 예시
///
/// 아래에서 Quarter(state)의 경우에 매칭한 결과의 일부분 state를 바인딩해서 사용가능.
/// 명시적으로 특수화도 가능함
///
fn value_in_us_cents(us_coin: USCoin) -> u8 {
    match us_coin {
        USCoin::Penny => 1,
        USCoin::Nickel => 5,
        USCoin::Dime => 10,
        USCoin::Quarter(UsState::Alabama) => {
            println!("what states make this? {:?}", UsState::Alabama);
            20
        }
        USCoin::Quarter(state) => {
            println!("what states make this? {:?}", state);
            25
        }
    }
}

/// ## `Option<T>` 이용한 매칭
/// match 표현식으로 `Option<T>`을 다뤄봄.
///
/// Some() => Some() : 같은 베리언트 가져서 가능
fn plus_one(x: Option<i32>) -> Option<i32> {
    // match x {
    //     None => None,
    //     Some(i) => Some(i + 1),
    // }
    x.map(|i| i + 1)
}

fn print_sample_of_plus_one() {
    let five = Some(5);
    println!(
        "five {:?} plus_one(five) {:?} plus_one(None) {:?}",
        five,
        plus_one(five),
        plus_one(None)
    );
}

/// ## 모든 가능성을 대처해야 하는 match 표현식
///
/// match 표현식에서 왠만해서 모든 가능한 경우를 러스트가 알려줌
#[allow(dead_code)]
struct ExhaustiveMatch;

enum Currency {
    US(USCoin),
    Base(Coin),
}

/// ## 포괄 패턴, other과 _ 자리표시자
///
/// match 표현식은 갈래를 순차적으로 평가하다가
/// 포괄적으로 받는 갈래를 마지막 차례에 둘 수 있다
///
/// 포괄 갈래의 변수를 안 쓰는 경우 이름을 _로 표시함.
///

fn catch_all_and_under_bar() {
    use rand::prelude::*;
    let wow = match thread_rng().gen_range(0..3) {
        1 => Some(Currency::US(USCoin::random_coin())),
        2 => Some(Currency::Base(Coin::random_coin())),
        other => {
            println!("{}, it other!", other);
            None
        }
    };

    match wow {
        Some(Currency::Base(coin)) => {
            println!("wow is coin {}", value_in_cents(coin))
        }
        Some(Currency::US(us)) => {
            println!("wow is uscoin {}", value_in_us_cents(us))
        }
        _ => {
            println!("it _!")
        }
    }
}

/// ## match보다 간단한 if let
fn other_if_let() {
    use rand::prelude::*;
    let wow = match random() {
        true => Some(Coin::random_coin()),
        other => {
            println!("{}, it other!", other);
            None
        }
    };

    if let Some(coin) = wow {
        if let Coin::Quarter = coin {
            println!("if let에서 쿼터 나옴")
        }
        println!("if let {:}", value_in_cents(coin));
    } else {
        println!("if let in else ");
    }
}
