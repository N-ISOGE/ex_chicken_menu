//! # 외부 의존성, 패키지, 파일 사용하기
//!
//! - [`use_extern_pkg`]
//! - [`use_nested_path`]
//! - [`IsolatingModule`]
//!

/// # 외부 패키지 사용하기
///
///  예시로 `rand`를 `Cargo.toml`에 의존성으로 추가하면 crate.io에서 의존성을 다운로드하고
///  프로젝트에서 `rand` 패키지를 사용 가능.
///
///  표준 라이브러리도 다른 외부 패키지처럼 `std` 패키지로 불러옴.
///
use rand::Rng;
#[allow(dead_code)]
pub fn use_extern_pkg() {
    let num = rand::thread_rng().gen_range(1..=100);
    use std::collections::HashMap;
    let nums_with_order = HashMap::from([(0, num)]);
    for (k, v) in nums_with_order.iter() {
        println!("{} | {}", k, v);
    }
}

/// # 중첩 경로, glob을 이용해서 대량의 use 나열을 정리하기
///
///  `use`사용해서 표현한 경로중에서 부모 모듈가 같은 경로는 중괄호로 묶을 수 있다.
///  self를 사용하면 부모 모듈을 나타내는 경로도 표현 가능함.
///
///  glob 연산자, `*`을 이용하면 하위에 있는 모든 공개 아이템을 가져올 수 있음.
///  주 사용처 예시
/// - test할 모든 아이템을 `tests` 모듈로 가져오는 용도.
/// - 프렐루드 패턴의 일부
#[allow(dead_code, unused)]
pub fn use_nested_path() {
    use std::env::*;
    use std::{
        io::{self, prelude},
        result,
    };
}

///  # 모듈을 다른 파일로 분리하기
///
///  모듈 파일은 한번만 로드되서 계속 참조됨.
///  - include 처럼 여러번 복사되는 것이 아님.
///
///  폴더로 된 패키지 불러오기
///  - 현재 `mod <folder_name, package_name>`
///  - 과거 `mod <folder_name, package_name>/mod.rs`
///
///  둘 다 가능하지만 사용할 땐 한쪽만 쓰기
#[allow(dead_code)]
struct IsolatingModule;

#[allow(dead_code)]
pub fn main() {}
