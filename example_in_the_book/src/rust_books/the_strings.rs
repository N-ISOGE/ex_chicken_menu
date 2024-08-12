//! # String
//!
//! ## 개요  
//!
//! - 문자열을 다루려할 때 만나는 특징
//!     - **발생할 수 있는 에러를 최대한 표시**하는 러스트의 성향
//!     - 문자열은 생각보다 **복잡한 자료구조**임.
//!     - **UTF-8**
//!         - 가변길이 인코딩, 파악할 때 문자소, 바이트 등... 나눠서 봐야 하는 점 등..
//!
//! **String은 바이트의 컬렉션**에 필요한 메소드가 추가되어 구현된 형태
//!
//! ## 요약
//!
//! 나중에 non ascii 문자를 다루는 고민을 하지 않아도 되지만
//! 문자열의 복잡한 모습을 보고 다루긴 해야 함.
//! 대신 이를 다룰 메소드도 있으니 확인.
//!
//! ## 문자열이란?  
//!
//! 언어에서 기본 제공하는 문자열 타입은 `str`, 문자열 슬라이스 하나임.
//! - **UTF-8**로 인코딩됨.  
//!
//! 표준 라이브러리에서 제공하는 건 `String` 타입, 특징: **가변적, 소유권, UTF-8**  
//!
//! 둘 다 많이 사용됨, **UTF-8** 강조.
//!
//! ## 문자열 생성하기
//!
//! - [`test_make_string`]
//!
//! ## 문자열 다루기
//!
//! - [`test_update_string`]
//! - [`test_oper_or_macro`]
//!
//! ## 문자열의 내부
//!
//! - [`test_internal_string`]
//!

use test_log::test;

/// # 문자열 생성하기
///
/// to_string(), from() 둘 다 같은 역할을 하는 중,
/// 스타일, 가독성을 고려하여 선택하자.
///
#[test]
pub fn test_make_string() {
    use std::any::{Any, TypeId};

    let make_form_new = String::new();
    debug_assert_eq!(TypeId::of::<String>(), make_form_new.type_id());

    let data = "초기화 값";
    let str_to_string = data.to_string();
    let literal_to_string = "초기화 값".to_string();
    debug_assert_eq!(literal_to_string, str_to_string);

    let init_string_used_from = String::from("초기화 값");
    debug_assert_eq!(str_to_string, init_string_used_from);
}

/// # 문자열 업데이트하기
///
#[test]
pub fn test_update_string() {
    let target = "foobar";

    let mut use_push_str = String::from("foo");
    use_push_str.push_str("bar");
    debug_assert_eq!(use_push_str, target);

    let mut use_other_str = String::from("foo");
    let other_str = "bar";
    use_other_str.push_str(other_str);
    debug_assert_eq!(use_other_str, target);

    let mut use_push = String::from("fooba");
    use_push.push('r');
    debug_assert_eq!(use_push, target);
}

/// # 연산자나 메크로 사용하여 문자열 다루기
///
/// 사람이 계산하는 상황을 생각하면 계산에서 쓰이는 인자는 쓰고 난 뒤 필요가 없음.
/// 그러면 인자를 나중에 쓰지 않는다고 한 뒤 계산의 결과를 쓰는 모습,
/// 소유권이 넘어가는 모습이 계산을 생각하기엔 편하다?
///
///
#[test]
pub fn test_oper_or_macro() {
    let target = "멸치액젓!푸훑";
    let report_word = String::from("멸치액젓!");
    let answer_word = String::from("푸훑");
    let total = report_word + &answer_word;
    debug_assert_eq!(total, target);

    // report_word 소유권 + &answer_word -> total

    use log::debug;
    use std::any::{self, Any};
    use std::ops::Add;

    fn check_type_name<T: ?Sized + Any>(_s: &T) -> &'static str {
        any::type_name::<T>()
    }

    debug!(
        "{:} \n\t fn add(self, s: &str) -> String",
        check_type_name(&String::add)
    );

    let s1 = String::from("깍");
    let s2 = String::from("두");
    let s3 = String::from("기");

    let use_format_macro = format!("{}.{}.{}", s1, s2, s3);
    let use_add_operator = s1 + "." + &s2 + "." + &s3;

    assert_eq!(use_add_operator, use_format_macro);
}

/// # 문자열의 내부
///
/// 단순하게 인덱스로 접근하는 방식은 에러 나옴.
///
/// 내부는 `Vec[u8]`로 구현되어 있지만
/// 이를 직접 접근하는건 의도와 맞지 않아 프로그래머를 햇갈리게 할 수 있다.
/// 유니코드 스칼라 값 != u8 값
///
///
#[test]
fn test_internal_string() {
    let s1 = String::from("안녕");
    // let h = s1[0];
    // the type `str` cannot be indexed by `{integer}`

    // let broken_char = &s1[0..2];
    let whole_char = &s1[0..3];
    assert_eq!("안", whole_char);

    for (number, c) in "안녕".chars().enumerate() {
        if let Some(ch) = &s1[number * 3..(number + 1) * 3].chars().take(1).next() {
            assert_eq!(c, *ch);
        };
    }

    for (number, b) in "안녕".bytes().enumerate() {
        log::debug!("안녕 {number} {b}");
    }
}
