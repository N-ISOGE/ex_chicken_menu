//!
//! ## Rust book
//!
//! 3. 배경지식  
//!     - [`number_game`]
//!     - [`the_background_knowledge`]
//!     - [`the_mutability_shadowing`]
//!
//! 4. 소유권  
//!     - [`the_ownership`]
//!     - [`the_refer_borrow`]
//!     - [`the_slice`]
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
//! 7. 모듈  
//! 모듈 기능은 코드를 나눠서 필요한 부분을 쉽게 찾고 사용할 수 있게 해줌.  
//! 일정 부분을 캡슐화해서 공개 범위를 정하는 방법을 제공함.  
//! 이름 충돌을 방지하는 스코프 기능도 소개함.  
//! 
//! 이 모듈 시스템의 주요 요소로 패키지, 크레이트, 코듈, 경로 등이 있음  
//! - 패키지: 카고 기능, 크레이트를 빌드, 테스트, 공유하는데 사용함.
//! - 크레이트: 라이브러리나 실행 가능한 모듈로 구성된 트리 구조.
//! - 모듈, use: 구조, 스코프를 제어, 세부 경로를 감추는 데 사용.
//! - 경로: 구조체, 함수, 모듈등의 이름을 지정함.
//! 
//! 상세 목록
//!     - [`the_package_and_crate`]
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
pub mod the_package_and_crate;

#[allow(dead_code)]
pub fn main() {
    expression_match::main();
    the_marcos::main();
}
