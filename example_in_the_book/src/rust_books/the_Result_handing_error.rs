//! # Result, 복구 가능한 에러 처리
//!
//! ## 개요
//!
//! 에러가 났지만 대응할 여지가 있다면 중단하지 않을 수 있다.
//! - 파일이 없어서 에러가 났다면 종료하는 대신 파일을 만드는 것을 고려할 수 있다.
//!
//! Result 열거형 모양
//! ```rust
//! enum Result<T, E> {
//!     Ok(T),
//!     Err(E),
//! }
//! ```
//!

#[test]
fn signature_of_result() {
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    use std::fmt;

    impl fmt::Display for Result<i32, String> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Result::Ok(num) => {
                    write!(f, "{}", num)
                }
                Result::Err(error_thing) => {
                    write!(f, "wow error {}", error_thing)
                }
            }
        }
    }

    debug_assert_eq!("1", format!("{}", Result::Ok(1)));
    debug_assert_eq!(
        "wow error wow",
        format!("{}", Result::Err(String::from("wow")))
    );
}
