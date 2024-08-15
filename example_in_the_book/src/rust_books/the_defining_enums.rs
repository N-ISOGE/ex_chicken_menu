//!
//! ## 열거형
//! > *열거형 예시, `Option` 열거형 살펴보기, `match` 식 사용, if let 표현*
//! - 열거형
//! - 구조체
//! - IP 주소로 예시
//!     - [`IpAddrKind`]
//! - 열거형 값
//!     - [`enums_varients`]
//! - 열거형 예시
//!     - [`defined_message_as_enum`]
//! - 열거형 예시 `std::option::Option`
//!     - [`option_enums`]
//!
#[allow(dead_code)]
pub fn main() {
    enums_basic();
    enums_varients();
    defined_message_as_enum();
    option_enums();
}

///
/// ## 열거형
/// - 하나의 타입이 가질 수 있는 배리언트를 열거함으로써 타입을 정의할 수 있도록함
///
/// ### 구조체와 비교
/// 관련된 것들을 묶는데 사용한 것이 구조체
/// 어떤 값이 여러개 가능한 값의 집합 중 하나라는 것을 표현하는 것이 열거형
///
/// ### 예시로 IP 주소 다루는 프로그램 만들어보기
/// 일단 IP 주소 표준이 IPv4, IPv6 존재 , v4,v6이라 하면
/// IP 종류라는 집합에 v4,v6이 존재, 이렇때 이것들을 열거한 열거형을 만드면?
/// 속성들이 서로 독립적으로 존재, 열거형은 집합 중 하나만 가능
///
enum IpAddrKind {
    V4,
    V6,
}

///
/// ## 열거형 값
/// 열거형으로 인스턴스를 만들어 보자
///
fn enums_basic() {
    let v_four = IpAddrKind::V4;
    let v_six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {
        match ip_kind {
            IpAddrKind::V4 => println!("V4!"),
            IpAddrKind::V6 => println!("V6!"),
        }
    }

    route(v_four);
    route(v_six);
}

///
/// - 열거형 베리언트 이용하기
/// 각 열거형 베리언트에 데이터를 직접 넣을 수 있다.
/// 이 때 각 베리언트 이름이 열거형 인스턴스의 생성자 함수처럼 기능함
///
/// 각각의 베리언트가 다른 타입, 다른 양의 연관된 데이터를 가질 수 있음.
///
///
fn enums_varients() {
    //  각 열거형 베리언트에 데이터를 직접 넣을 수 있다.
    enum IpAddr {
        V4(String),
        V6(String),
    }

    // 각 베리언트 이름이 열거형 인스턴스의 생성자 함수처럼 됨.
    // IpAddr::V4() -> String 인수를 받아 인스턴스를 만드는 함수
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("IpAddr");
    if let IpAddr::V4(name) = home {
        println!("home {}", name);
    }
    if let IpAddr::V6(name) = loopback {
        println!("loopback {}", name);
    }

    // 각각의 베리언트가 다른 타입, 다른 양의 연관된 데이터를 가질 수 있음.
    enum IpAddrVariation {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrVariation::V4(127, 0, 0, 1);

    let loopback = IpAddrVariation::V6(String::from("::1"));

    println!("IpAddrVariation");
    if let IpAddrVariation::V4(a, b, c, d) = home {
        println!("home {:0x} {:0x} {:0x} {:0x}", a, b, c, d);
    }
    if let IpAddrVariation::V6(name) = loopback {
        println!("loopback {}", name);
    }

    // 표준에서 이미 정의해놨음.
    // 어떻게 구현했나?
    {
        use std::net;
        let home = net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1));
        let loopback = net::IpAddr::V6(net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        println!("std::net::IpAddr");
        println!("{:?} {:?}", home, loopback);
    }
}

///
/// ## 열거형 예시
/// - `Message` 열거형
///     - `Quit`은 아무 연관 데이터가 없음
///     - `Move`는 구조체처럼 이름이 있는 필드있음
///     - `Write`는 `String` 가짐
///     - `ChangeColor`는 3개의 `i32`를 가짐
///
/// 다른 구조체 여러개가 아닌 하나의 열거형에서 이를 모아서 볼 수 있다
///
/// ### 구현 가능
/// 구조체처럼 impl에서 정의할 수 있다.
///
#[allow(unused)]
fn defined_message_as_enum() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
}

///
/// ## 열거형 예시 `std::option::Option`
///
/// 표준 라이브러리에서 열거형으로 정의된 `Option`소개
/// - 값이 있거나 없을 수 있는 상황에 쓰임
///
/// 있는지 확인할 때 처리할 경우의 수를 다 따져 봤는지 컴파일러가 알려줌.
///
/// null 개념을 다르게 구현함.
///
/// ### 근데 비슷한 개념이 Option에 있긴하다
/// 하지만 이 None이 null과 다른건, 컴파일러에서 다루는 방식이 차이가 있다
/// option은 None이 될 수 있다는 경고를 컴파일러에서 받음
///
/// 관련된 메서드도 많다. 참고해서 활용하기
/// None, T 두 경우를 다루기 위한 도구 match 표현식 존재
///
#[allow(unused)]
fn option_enums() {
    enum Option<T> {
        None,
        Some(T),
    }

    // option 예시
    let some_number = Some(5);
    let some_char = Some('c');

    let five: i32 = 5;
    let absent_number: std::option::Option<i32> = Some(4);
    // 일단은 다른 타입, 같다고 무심코 작성하는 걸 방지함
    // let sum = five + absent_number;
    // 널 체크를 언어적으로 의식하게 해줌.
    let sum = match absent_number {
        None => five,
        Some(i) => i + five,
    };
}
