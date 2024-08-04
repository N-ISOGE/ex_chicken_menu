//! # vector
//!
//! contiguous 메모리 공간 제공, 같은 타입 여러개 저장
//!

use test_log::test;

/// # 벡터 만들고 업데이트 하기
///
/// ## 벡터 만들기
/// 어떤 값을 저장하는 지 모르니깐 타입을 명시해둠. -> \<타입 인자\>
/// - 아니면 저장할 원소로 추론하게 함.
/// - 메크로 `vec![ <저장할 원소>, ...]`도 제공
///
/// ## 벡터 업데이트 하기
///
/// `push()`를 사용해서 벡터를 변경 가능, 대신 `mut`로 변경 가능한 것로 선언해야함.
/// 타입을 명시하지 않아도 `push()`로 삽입한 원소 타입이 `i32`인 것으로 러스트가 타입을 추론함.

#[allow(clippy::vec_init_then_push)]
#[test]
pub fn make_update_vector() {
    use log::debug;
    use std::any::{Any, TypeId};

    let v: Vec<i64> = Vec::new();
    debug_assert_eq!(v.type_id(), TypeId::of::<Vec<i64>>());

    let v = vec![1, 2, 3];
    debug_assert_eq!(v.type_id(), TypeId::of::<Vec<i32>>());

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    debug!("{:?}", v);
    debug_assert_eq!(v, [5, 6, 7, 8]);
}

/// # 벡터 요소 읽기
///
/// 요소를 참조하는 방법으로 인덱싱, `get()`이 있다.
/// - 인덱싱 : 참조자 반환함. 경계를 넘은 값을 참조하면 panic함.
/// - `get()` : `Option<&T>` 반환함. 경계를 넘은 값을 참조하면 None 반환.
///
/// borrow 검사 생각하면 불변, 가변 참조자 고려해봐야 함.
/// 벡터 재할당 때문에 불변 참조자를 넘겨 받게 만들어둠.
#[allow(clippy::useless_vec)]
#[test]
pub fn read_element_of_vector() {
    use log::debug;

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    debug!("{third}, the third element is referenced by index number");
    assert_eq!(*third, 3, "v[2] is {}", v[2]);

    let third: Option<&i32> = v.get(2);
    match third {
        None => {
            panic!("there is no third element");
        }
        Some(number) => {
            debug!("{number}, the third element is referenced by get method");
            assert_eq!(*number, 3, "v[2] is {}", v[2]);
        }
    }

    let first = &v[0];
    // v.push(6);
    debug!("the first element : {first}");
}

/// # 반복문으로 벡터 방문하기
/// 
/// 
#[allow(clippy::useless_vec)]
#[test]
pub fn vector_iteration() {
    use log::debug;

    let v = vec![100, 37, 57];
    let mut vector_log = String::new();
    for num in &v {
        vector_log += format!("{} ", num).as_str();
    }
    debug!("{}", vector_log);

    let mut v = v;
    for i in &mut v {
        *i += 50;
    }
    assert_eq!(v, [150, 87, 107]);
}
