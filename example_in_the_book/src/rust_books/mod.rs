//!
//! ## Rust book
//!
//! 3. 배경지식
//!     - [`number_game`]
//!     - [`the_background_knowledge`]
//!     - [`the_mutability_shadowing`]
//!
//!
//! 4. 소유권
//!     - [`the_ownership`]
//!     - [`the_refer_borrow`]
//!     - [`the_slice`]
//!
//!
//! 5. 구조체
//!     - [`the_defining_structs`]
//!     - [`the_example_of_using_structs`]
//!     - [`the_method_syntax`]
//!
//! 6. 열거형
//!     - [`defining_enums`]
//!     - [`expression_match`]
//!
pub mod defining_enums;
pub mod expression_match;
pub mod number_game;
pub mod the_background_knowledge;
pub mod the_defining_structs;
pub mod the_example_of_using_structs;
pub mod the_marcos;
pub mod the_method_syntax;
pub mod the_mutability_shadowing;
pub mod the_ownership;
pub mod the_refer_borrow;
pub mod the_slice;

#[allow(dead_code)]
pub fn main() {
    expression_match::main();
    the_marcos::main();
}
