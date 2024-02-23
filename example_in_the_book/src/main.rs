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

pub mod number_game;
pub mod the_background_knowledge;
pub mod the_defining_structs;
pub mod the_example_of_using_structs;
pub mod the_method_syntax;
pub mod the_misstakes_by_newbies;
pub mod the_mutability_shadowing;
pub mod the_ownership;
pub mod the_refer_borrow;
pub mod the_slice;

///
/// # rust, 러스트
///
///
/// 3. 배경지식
///     - [`number_game`]
///     - [`the_background_knowledge`]
///     - [`the_mutability_shadowing`]
///
///
/// 4. 소유권
///     - [`the_ownership`]
///     - [`the_refer_borrow`]
///     - [`the_slice`]
///
///
/// 5. 구조체
///     - [`the_defining_structs`]
///     - [`the_example_of_using_structs`]
///     - [`the_method_syntax`]
///
///
/// ex. 
/// 1. 실수  
///     - [`the_misstakes_by_newbies`]
///
fn main() {
    the_background_knowledge::main();
}
