//!
//! # Rust book
//!
//! ## 3. 배경지식
//!  - [`number_game`]
//!  - [`the_background_knowledge`]
//!  - [`the_mutability_shadowing`]
//!
//! ## 4. 소유권
//!  - [`the_ownership`]
//!  - [`the_refer_borrow`]
//!  - [`the_slice`]
//!
//! ## 5. 구조체
//!  - [`the_defining_structs`]
//!  - [`the_example_of_using_structs`]
//!  - [`the_method_syntax`]
//!
//! ## 6. 열거형
//!  - [`the_defining_enums`]
//!  - [`the_expression_match`]
//!
//! ## 7. 모듈
//!  모듈 기능은 코드를 나눠서 필요한 부분을 쉽게 찾고 사용할 수 있게 해줌.
//!  일정 부분을 캡슐화해서 공개 범위를 정하는 방법을 제공함.
//!  이름 충돌을 방지하는 스코프 기능도 소개함.
//!
//!  이 모듈 시스템의 주요 요소로 패키지, 크레이트, 코듈, 경로 등이 있음
//!  - 패키지: 카고 기능, 크레이트를 빌드, 테스트, 공유하는데 사용함.
//!  - 크레이트: 라이브러리나 실행 가능한 모듈로 구성된 트리 구조.
//!  - 모듈, use: 구조, 스코프를 제어, 세부 경로를 감추는 데 사용.
//!  - 경로: 구조체, 함수, 모듈등의 이름을 지정함.
//!
//!  **상세 목록**
//!  - [`the_package_and_crate`]
//!  - [`the_extern_pkg_and_pkg_file`]
//!
//! ## 8. 컬렉션
//!
//!  컬렉션, 러스트 표준 라이브러리에서 제공하는 데이터 구조들 중 한 종류.
//!  - 여러 값을 담을 수 있고 힙을 사용함.
//!  - 크기와 비용이 다름.
//!  소개할 컬렉션으로는 vector, string, hash map이 있고 다른 것은 표준 문서 참조.
//!
//!  **상세 목록**
//!  - [`the_vector`]
//!  - [`the_strings`]
//!  - [`the_hashmap`]
//!  - [`the_middle_exam_collect`]
//!
//! ## 9. 에러 처리
//!
//! 에러의 종류를 복구 가능한 에러와 복구 불가능한 에러, 둘로 나눠둠.
//! 복구 가능 에러는 사용자에게 알리고 다시 명령를 하는 상황을 생각하면 됨.
//! 복구 불가능한 에러는 바로 프로그램을 종료해야 하는 상황을 생각하면 됨.
//! 러스트는 예외 처리 기능이 없음.
//! 대신 복구 가능한 에러를 위한 `Result<T, E>`형식,
//! 복구 불가능한 에러에 종료시켜버리는 `panic!` 메크로가 존재.
//!
//! ### 에러 처리 상세 목록
//! - [`the_panic_handling_error`]
//! - [`the_result_handing_error`]
//! - [`the_panic_or_something_else`]
pub mod number_game;
pub mod the_background_knowledge;
pub mod the_defining_enums;
pub mod the_defining_structs;
pub mod the_example_of_using_structs;
pub mod the_expression_match;
pub mod the_extern_pkg_and_pkg_file;
pub mod the_hashmap;
pub mod the_marcos;
pub mod the_method_syntax;
pub mod the_middle_exam_collect;
pub mod the_mutability_shadowing;
pub mod the_ownership;
pub mod the_package_and_crate;
pub mod the_panic_handing_error;
pub mod the_refer_borrow;
pub mod the_result_handing_error;
pub mod the_slice;
pub mod the_strings;
pub mod the_vector;
pub mod the_panic_or_something_else;