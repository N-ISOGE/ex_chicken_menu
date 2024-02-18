//!
//! # rust, 러스트
//!
//! cargo 좋다
//! utf-8를 기본적으로 쓰는 거 좋다
//!
//! ### 실수
//! #### 필요한 참조만 쓰기
//! - 매개변수를 참조자로 쓰는 일이 있을 때
//!
//! 참조자를 쓰지 않아도 알아서 추론해서 슬라이스로 넘긴다       
//!
//! #### 슬라이스 인덱싱을 적당하게 사용하기
//!
//! 쉽지만 쓰다가 문제생김 (범위를 벗어난 접근)
//! map으로 보내서 알아서 범위를 정하게 함
//!
//! #### sentinel value, 초계값, 끝값?
//!
//! 함수 끝날 때, 특정한 조건에서 특정한 값을 쓰는 일이 있다.
//! - 빈 값을 반환하는 경우에 "", -1, null
//! 그 대신에 `Option<>`을 사용
//!
//! #### Enum 쓰기
//!
//! 쓰면 명확해짐
//!
//! 패턴 매칭?
//! - 어떤 조건을 가정하기 위해 코드를 여러개 쓰지 않고 한번에 처리
//! - 조건문에 패턴을 넣어 확인?
//!
//! #### 에러 핸들링 하기
//!
//! 물음표 연산자로 에러 확인하기
//! 개인적으로 만든 에러 타입을 구현 하기
//! - 메크로를 이용
//!
//! #### standard lib 트레이트 사용
//!
//! From, TryFrom
//! FromStr
//!
//! #### standard lib 메크로 활용하기
//!
//! todo
//! concat, format
//!
//! #### 도구 활용하기
//! - cargo fmt
//! - cargo-clippy
//!
//! #### 코드 구조
//! Rc, Arc
//!
//!
//! ### 의존성
//! - rust-update
//!     - libiconv
//!     - pkg-config
//!     - openssl
//!
//!
//!

// 3. 배경지식
pub mod number_game;
pub mod the_background_knowledge;
pub mod the_mutability_shadowing;
// 4. 소유권
pub mod the_ownership;
pub mod the_refer_borrow;

///
/// # rust, 러스트
///
pub fn main() {
    the_refer_borrow::main();
}
