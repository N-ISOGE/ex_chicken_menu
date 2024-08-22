//! # hello world
//!
//! ## 특정 형식으로 출력
//!
//! - [`formatted_print`]
//!
//! ## `fmt::Debug`가 구현된 형식 출력
//!
//! - [`formatted_debug_print`]
//!
//! ## `fmt::Display`
//!
//! - [`impl_display_for_type`]
//!
//! ## `fmt::Display` 다양한 상황에서 구현해보기
//!
//! - [`impl_display_fot_vec`]
//!
//! ## formatting 나온거 다 써보기
//!
//! - [`compound_fmt_display`]
//!

use test_log::test;

// noinspection ALL
// no ALL
#[test]
fn formatted_print() {
    // 보통 `{}`을 사용하면 대부분의 인자가 대체된다.
    debug_assert_eq!("31 days", format!("{} days", 31));

    // 위치를 정한 인자도 가능,
    // `{}` 내부에 정수를 입력하면 숫자에 맞는 순서의 인자로 대체됨.
    // 순서는 0부터 시작함.
    debug_assert_eq!(
        "푸른 하늘에 밝은 해, 밝은 형광등에 푸른 문서첩",
        format!("{1} 하늘에 {0} 해, {0} 형광등에 {1} 문서첩", "밝은", "푸른")
    );

    // 이름도 가능
    debug_assert_eq!(
        "흑미밥 멸치볶음 순두부찌개",
        format!(
            "{밥} {반찬} {찌개}",
            밥 = "흑미밥",
            반찬 = "멸치볶음",
            찌개 = "순두부찌개"
        )
    );

    // 다른 형식 지정자를 이용해서 형식을 구성 가능
    // `:` 사용
    debug_assert_eq!(
        // 기본 십진법
        format!("{}", 14523),
        "14523"
    );
    debug_assert_eq!(
        // 이진법, binary
        format!("{:b}", 14523),
        "11100010111011"
    );
    debug_assert_eq!(
        // 8진법, octal
        format!("{:o}", 14523),
        "34273"
    );
    debug_assert_eq!(
        // 16진법, hexa
        format!("{:x}", 14523),
        "38bb"
    );

    // 특정 길이를 지정하면서 오른쪽, 왼쪽 정렬도 가능, 특정 문자로 채우기 가능
    debug_assert_eq!(format!("{숫자:>5}", 숫자 = 1), "    1");
    debug_assert_eq!(format!("{숫자:0>5}", 숫자 = 1), "00001");
    debug_assert_eq!(format!("{숫자:0<5}", 숫자 = 1), "10000");
    debug_assert_eq!(
        // 특정 부분에서는 형식 지정자로 쓸려면 마지막에`$`를 붙여야 함.
        format!("{number:0>width$}", number = 1, width = 5),
        "00001"
    );

    // 모든 인자가 있는지 rust 자체에서 확인함
    // println!("My name is {0}, {1} {0}", "Bond");
    //   FIXME ^ Add the missing argument: "James"

    // 형식으로 출력하기 위해선 대상 타입이 `fmt::Display`가 구현되어 있어야 함.
    // `{:?}`의 경우 `fmt::Debug`가 구현되어 있어야 함.

    // 이미 있는 변수를 직접적으로 대체하는 인자 취급할 수 있음.
    let number: f64 = 1.0;
    let width: usize = 5;
    debug_assert_eq!(format!("{number:>width$}"), "    1");

    // 파이 출력해보기

    let pi = std::f64::consts::PI;
    debug_assert_eq!(
        // IEEE 753 부동소수점 기본 방식 :
        // round half to even, round to nearest even, 오사오입
        // 가장 가까운 짝수로의 자리맞춤, 반올림
        // 버리는 부분이 정확히 절반이면서
        // 최소 단위가 홀수이면 올려서, 짝수면 버려서 짝수로 만듬.
        // 3.141(5) -> 최소 단위 1, 홀수에 버리는 부분이 절반 5
        format!("Pi is roughly {pi:1.3}"),
        "Pi is roughly 3.142"
    );
}

#[test]
fn formatted_debug_print() {
    // `fmt::Debug` 트레이트는 `fmt::Display`가 구현이 필요하지 않은 타입에 대해
    // derive를 사용해서 자동으로 구현 가능
    // `std` 라이브러리에 있는 타입은 자동적으로 `{:?}`로 출력 가능
    #[derive(Debug)] // 이 attribute 사용함.
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Deep(DebugPrintable);

    debug_assert_eq!(
        format!("{:?} months in a year.", 12),
        "12 months in a year."
    );

    debug_assert_eq!(
        format!("Now {:?} will print!", DebugPrintable(3)),
        "Now DebugPrintable(3) will print!"
    );
    debug_assert_eq!(
        format!("Now {:#?} will print!", DebugPrintable(3)),
        "Now DebugPrintable(\n    3,\n) will print!"
    );
    debug_assert_eq!(
        format!("Now {:?} will print!", Deep(DebugPrintable(7))),
        "Now Deep(DebugPrintable(7)) will print!"
    );

    // `{:#?}`을 이용하면 "pretty printing", 보기 좋게 출력됨

    let debug_sample = DebugPrintable(3);
    let deep_sample = Deep(debug_sample);

    debug_assert_eq!(
        format!(
            "\n{:#?}\n{:?}\n{:?}\n{}",
            deep_sample, deep_sample, deep_sample.0, deep_sample.0 .0
        ),
        r"
Deep(
    DebugPrintable(
        3,
    ),
)
Deep(DebugPrintable(3))
DebugPrintable(3)
3"
    );
}

#[test]
fn impl_display_for_type() {
    // `fmt::Display`는 제네릭하게 대상을 다루는데는 구현되어 있지 않음.
    // - 어떤 대상을 형식적으로 출력할지 모르는데 왜 구현하나?
    //
    // 구현할 대상이 정해져 있으면 대상에 대해 구현하면 됨

    use std::fmt;

    #[derive(Debug)] // `Debug`는 구현하지만 `Display`와 별개임.
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);

    debug_assert_eq!(format!("Display: {}", minmax), "Display: (0, 14)");
    debug_assert_eq!(format!("Debug: {:?}", minmax), "Debug: MinMax(0, 14)");

    let point = Point2D { x: 5.2, y: 7.6 };

    debug_assert_eq!(format!("Display: {}", point), "Display: x: 5.2, y: 7.6");
    debug_assert_eq!(
        format!("Debug: {:?}", point),
        "Debug: Point2D { x: 5.2, y: 7.6 }"
    );

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    debug_assert_eq!(format!("Display: {}", complex), "Display: 3.3 + 7.2i");
    debug_assert_eq!(
        format!("Debug: {:?}", complex),
        "Debug: Complex { real: 3.3, imag: 7.2 }"
    );
}

#[test]
fn impl_display_fot_vec() {
    // 여러 원소들을 가진 자료형에 대해 `fmt::Display`을 구현하는 것은
    // 약간 난해함.
    // `write!`는 `fmt::Result`를 반환하는데 이를 어떻게 처리하나?
    // `?`연산자를 뒤에 붙이면 에러를 반환한 경우 이를 반환하고 끝남.
    // 아니면 다음 문장으로 넘어감.
    use std::fmt;

    struct List(Vec<i64>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }

            write!(f, "]")
        }
    }

    let list = List(vec![1, 2, 3]);

    debug_assert_eq!(format!("{}", list), "[0: 1, 1: 2, 2: 3]");
}

#[test]
fn compound_fmt_display() {
    use std::fmt;

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "RGB ({red}, {green}, {blue}) 0x{red:0>2X}{green:0>2X}{blue:0>2X}",
                red = self.red,
                green = self.green,
                blue = self.blue
            )
        }
    }

    let color = Color {
        red: 9,
        green: 243,
        blue: 245,
    };

    debug_assert_eq!(format!("{}", color), "RGB (9, 243, 245) 0x09F3F5");
}
