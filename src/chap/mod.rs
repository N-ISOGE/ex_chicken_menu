//!
//! # rust, 러스트
//!
//! cargo 좋다
//! utf-8를 기본적으로 쓰는 거 좋다
//!
//! ### 의존성
//! - rust-update
//!     - libiconv
//!     - pkg-config
//!     - openssl
//!

// 3. 배경지식
pub mod number_game;
pub mod the_background_knowledge;
pub mod the_mutability_shadowing;

// 4. 소유권
pub mod the_ownership;
pub mod the_refer_borrow;
pub mod the_slice;

// 5. 구조체
pub mod the_defining_structs;
pub mod the_method_syntax;

// e-0. 실수
pub mod the_misstakes_by_newbies;

/// # rust, 러스트
///
pub fn main() {
    the_method_syntax::main();
}
