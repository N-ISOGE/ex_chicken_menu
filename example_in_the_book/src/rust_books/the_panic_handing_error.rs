//! # panic!, 복구 불가능한 에러 처리
//!
//! ## 개요
//!
//! 대처할 방법이 없으면 쓰는 메크로, `panic!`
//! 패닉을 일으키는 방법: 코드가 패닉을 일으키는 동작을 함, `panic!` 사용하기
//! 패닉이 일어나면? -> 실패 메시지 출력, unwind, 스택 청소, 종료
//!
//! > ### unwind, 되감기를 할지 말지 선택하기
//! > `panic!`을 할 때 프로그램이 스택을 청소하게 하거나 안하는 `about`, 그만둘 수 있다.
//! > Cargo.toml에 `[profile]`섹션에 `panic = 'about'`를 추가하면 그만두게 만들 수 있음.
//!
//! 환경 변수를 정의해두면 호출 스택을 출력 가능
//!
//!
//! 실제로 패닉 사용해 보기
//! - [`example_of_panic`]
//!
//! ## 백트레이스 이용하기
//!
//! - [`test_use_panic_backtrace`]
//!
//! 백트레이스를 보려면 백트레이스 관련 환경변수 `RUST_BACKTRACE`을 설정하면 됨.
//! 백트레이스: 어떤 지점에 도달하기까지 호출한 모든 함수의 목록
//!
//! `panic!`을 쓸지 말지는 나중에 다룸.
//!

use test_log::test;

#[test]
#[should_panic(expected = "crash")]
fn example_of_panic() {
    panic!("crash and burn");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_use_panic_backtrace() {
    let v = vec![1, 2, 3];

    v[99];
}
