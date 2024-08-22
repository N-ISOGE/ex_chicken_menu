//! # 기본형들
//!
//! ## scalar(scale-er) types
//!
//! 부호 있는 정수 : `i<size: 비트수로 표현>`
//! 부호 없는 정수 : `u<size: 비트수로 표현>`
//! 부동 소수점 : `f32`, `f64`
//! 유니코드 scalar 값 : `char` <- 값의 크기에 따라 타입 크기가 가변
//! `bool`
//! 단위(unit) 타입 `()` <- 값은 비어있는 튜플만 가짐
//!
//! ## 복합 타입
//!
//! 배열 : `[사과, 배, 포도]`
//! 튜플 : `(이름, 나이, 주소)`
//!
//! > [!NOTE] 변수 타입을 명시할 수 있음
//!
//! 예시
//! ```rust
//! fn test_primitives_example() {
//!     // bool 타입
//!     let _logical: bool = true;
//!
//!     // 타입 어노테이션
//!     let _annotation_float: f64 = 1.0;
//!     // 리터럴에 접미사 연산자를 붙여 표현함.
//!     let _literal_float = 1.0f64;
//!
//!     // 명시되지 않은 정수형, 실수형의 타입 예시
//!     let default_float_type_is = 1.0;
//!     let default_integer_type_is = 1;
//!
//!     use std::any::{Any, TypeId};
//!     debug_assert_eq!(default_integer_type_is.type_id(), TypeId::of::<i32>());
//!     debug_assert_eq!(default_float_type_is.type_id(), TypeId::of::<f64>());
//!
//!     // 사용하는 것에 따라 타입을 결정함.
//!     let mut inferred_type = 12;
//!     inferred_type = 4294967296i64;
//!     debug_assert_eq!(inferred_type.type_id(), TypeId::of::<i64>());
//! }
//! ```
//!
//! ## 리터럴과 연산자들
//!
//! 여러 기본 타입(정수, 실수, 유니코드 스칼라값, 문자열, 논리, unit)은
//! 리터럴로 표현할 수 있다.
//! - 정수는 타입 접미사붙여서 이진 팔진 십육진 선택 가능
//!
//! 숫자 리터럴에선 가능한거
//! - 리터럴 중간에 언버스코어를 넣어서 분리 가능
//!     - `10000` -> `1_0000`
//! - e 표기법 가능
//!     - `200` -> `2e2`
//!
//! ```rust
//! //noinspection RsSimplifyBooleanExpression
//! #[test]
//! fn literals_and_operators() {
//!     // 정수 덧셈
//!     debug_assert_eq!(-1, 1i32 - 2);
//!
//!     // 부호
//!     debug_assert_eq!("-1", format!("{}", 1i32 - 2i32));
//!
//!     // e 표기법
//!     debug_assert_eq!("10000 -0.0025", format!("{} {}", 1e4, -2.5e-3));
//!
//!     // 논리 표현식들
//!     debug_assert_eq!(true, true && true);
//!     debug_assert_eq!(false, !true);
//!
//!     // 비트 연산자
//!     debug_assert_eq!(0x0B, 0x0A | 0x01);
//!     debug_assert_eq!(0x48, 0x7A & 0xC9);
//!     debug_assert_eq!(0xAA, 0xA0 ^ 0x0A);
//!     debug_assert_eq!(0x40, 1 << 6);
//!
//!     // 숫자 사이에 구분자 넣어주기
//!     debug_assert_eq!(
//!         1000000000000000000000000u128,       // 없는 경우
//!         1_0000_0000_0000_0000_0000_0000u128  // 있는 경우
//!     );
//! }
//! ```
//!
//! ## 튜플
//!
//! > [!NOTE] 다른 타입의 값을 모아둔 콜렉션
//!
//! 모양새는 `( T1, T2, ... )`
//! - 소괄호 사용
//! - 타입을 나열함
//!
//! 함수에서 여러 값을 동시에 반환할 때 사용 가능.

use test_log::test;

#[test]
fn show_usage_of_tuple() {
    // 매개변수로 받을 때나 반환할 때나 여러값 동시에 전달
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // c++에서 Structured binding declaration라고 부름
        let (int_param, bool_param) = pair;

        (bool_param, int_param)
    }

    // 여러 타입이 섞인 튜플
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    debug_assert_eq!(1u8, long_tuple.0);
    debug_assert_eq!(2u16, long_tuple.1);

    debug_assert_eq!(
        "(1, 2, 3, 4, -1, -2, -3, -4, 0.1, 0.2, 'a', true)",
        format!("{:?}", long_tuple)
    );

    let tuple_of_tuples = ((1, 2, 3), (3isize, true), "123");

    debug_assert_eq!(
        r#"((1, 2, 3), (3, true), "123")"#,
        format!("{:?}", tuple_of_tuples)
    );

    // 튜플의 타입 갯수 한계를 12개로 정해둠.
    // let default_by_marco = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // debug_assert_eq!(
    //     r"(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13)",
    //     format!("{:?}", default_by_marco)
    // );

    let pair = (3, true);
    debug_assert_eq!("(3, true)", format!("{:?}", pair));
    debug_assert_eq!("(true, 3)", format!("{:?}", reverse(pair)));

    use std::any::{Any, TypeId};
    // 원소 하나인 튜플을 표현할 때는 원소 뒤에 쉼표를 붙여두기
    debug_assert_eq!(TypeId::of::<(i32,)>(), (3,).type_id());
    debug_assert_eq!(TypeId::of::<i32>(), (3).type_id());

    let tuple = (10, "이", 0.3, false);
    let (a, b, c, d) = tuple;

    debug_assert_eq!(
        r#"10 "이" 0.3 false"#,
        format!("{:?} {:?} {:?} {:?}", a, b, c, d)
    );

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    use std::fmt;

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }

    let mat = Matrix(1.1, 1.2, 2.1, 2.2);

    debug_assert_eq!("( 1.1 1.2 )\n( 2.1 2.2 )", format!("{:}", mat));

    fn transpose(matrix: &Matrix) -> Matrix {
        Matrix {
            0: matrix.0,
            1: matrix.2,
            2: matrix.1,
            3: matrix.3,
        }
    }
    debug_assert_eq!("( 1.1 2.1 )\n( 1.2 2.2 )", format!("{:}", transpose(&mat)));
}
