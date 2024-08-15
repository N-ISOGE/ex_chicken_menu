//! # hash map
//!
//! ## 개요
//!
//! 모양은 `HashMap<K,V>`로 키와 값에 대해 해시함수로 매핑한 것임.
//! - 다르게 부르는 걸로 해시, 맵, 오브젝트, 해시 테이블, associative(연관) 배열 등...
//!
//! 인덱스 대신 임의의 타입으로 정의한 키를 이용해서 데이터 찾음.
//!
//! 더 찾을 거면 표준 라이브러리 문서 참고.
//!
//! ## 해시맵 만들기
//! > [`test_make_manipulate_hashmap`]
//!
//! 자주 사용하는 건 아니라 std::prelude에 없어서 따로 모듈을 불러와야 함.
//! 힙 사용함.
//!
//! ## 접근하기
//! > [`test_make_manipulate_hashmap`]
//!
//! for로 반복 접근 가능.
//!
//! ## 해시맵과 소유권
//! > [`test_hashmap_and_ownership`]
//!
//! 해시맵에 변수를 이동시킨 뒤에는 소유권이 해시맵으로 넘어감.
//!
//! ## 해시맵 업데이트 하기
//! > [`test_update_hashmap`]
//!
//! ## 해시함수
//!
//! `BuildHasher` 트레이트가 구현된 타입, hasher가 해싱을 담당
//!

use test_log::test;

#[test]
fn test_make_manipulate_hashmap() {
    //
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("청팀"), 10);
    scores.insert(String::from("황팀"), 50);

    use std::any::{Any, TypeId};
    debug_assert_eq!(scores.type_id(), TypeId::of::<HashMap<String, i32>>());

    //
    let team_name = String::from("청팀");
    let result_score = scores.get(&team_name).copied().unwrap_or(0);
    let team_name = String::from("적팀");
    let not_exist_result_score = scores.get(&team_name).copied().unwrap_or(0);

    debug_assert_eq!(result_score, 10);
    debug_assert_eq!(not_exist_result_score, 0);

    for (key, value) in &scores {
        debug_assert_eq!(*value, scores.get(key).copied().unwrap_or(0));
    }
}

#[test]
fn test_hashmap_and_ownership() {
    use std::collections::HashMap;

    let field_name = String::from("좋아하는 음식");
    let field_value = String::from("냉면");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    // log::debug!("{}", field_name);
}

//noinspection RsDuplicateHashKey
#[test]
fn test_update_hashmap() {
    use std::collections::HashMap;

    let mut routine = HashMap::new();

    //
    routine.insert(String::from("물 마시기"), 80);
    routine.insert(String::from("물 마시기"), 50);

    debug_assert_eq!(routine.get("물 마시기").copied().unwrap_or(0), 50);

    //
    routine.insert(String::from("청소"), 70);

    routine.entry(String::from("식사 준비")).or_insert(60);
    routine.entry(String::from("청소")).or_insert(50);

    debug_assert_eq!(routine.get("청소").copied().unwrap_or(0), 70);
    debug_assert_ne!(routine.get("식사 준비").copied().unwrap_or(0), 0);

    //
    let text = r"
    1 1 2 3 5 8 1 3 2 1 3 4 5 5 8 9 1 4 4 2 3 3 3
    7 7 6 1 0 9 8 7 1 5 9 7 2 5 8 4 4 1 8 1";

    let mut num_map = HashMap::new();

    for word in text.split_whitespace() {
        num_map
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(0);
    }

    debug_assert_eq!(num_map.get("1").copied().unwrap_or(0), 9);
    // log::debug!("{:?}",num_map);
}
