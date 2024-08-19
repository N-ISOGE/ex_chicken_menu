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
//! # use test_log::test;
//!
//! #[test]
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
