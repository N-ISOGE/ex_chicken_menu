// 3. 배경지식
pub mod the_background_knowledge;
pub mod the_mutability_shadowing;
pub mod number_game;
// 4. 소유권
pub mod the_ownership;
pub mod the_refer_borrow;

/// 
/// # rust, 러스트
/// 
/// cargo 좋다 
/// utf-8를 기본적으로 쓰는 거 좋다
/// 
pub fn main() {
    the_refer_borrow::main();
}
