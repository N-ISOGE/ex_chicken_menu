//!
//! # 슬라이스
//! - 슬라이스?
//!     - [`not_using_slice`]
//! - 예시로 든 문자열 슬라이스
//!     - [`using_string_slice`]
//! - 슬라이스 관련 상호작용
//!     - [`slice_related_action`]
//!
//!
#[allow(unused)]
pub fn main() {
    println!("참조와 대여 the_refer_borrow -> 슬라이스 the_slice");
    // - 슬라이스?
    not_using_slice();
    // - 예시로 든 문자열 슬라이스
    using_string_slice();
    // - 슬라이스 관련 상호작용
    slice_related_action();
    println!("슬라이스 the_slice -> 구조체란? the_defining_structs");
}

/// ## 슬라이스?
/// 슬라이스는 컬렉션의 연속된 일련의 요소를 참조, **참조자**임.
///
/// 슬라이스 안쓰는 예시
fn not_using_slice() {
    // 문자열의 첫 번째 단어를 반환하는 함수
    // 문자열의 일부 -> 단어가 끝나는 위치, 인덱스를 반환하나?
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }

    // 위에 있는 함수 사용
    let mut s = String::from("이것도 작동 하나");
    let word = first_word(&s);
    // 가리키는 곳을 바꾸면?
    s.clear();
    // word는 의미가 없음. 프로그래머들이 알아서 의미를 부여하면서 관리해야 함.
    println!("'{}' 에 {}가 무슨 의미가 있나", s, word);
}
///
/// 프로그래밍 언어가 참조에 대한 정보를 관리하게 만들 수 없을까?
///
/// ## 예시로 든 문자열 슬라이스
///  
/// 문자열 슬라이스는 문자열의 일부를 가리키는 슬라이스, 참조자를 말함
///
/// (숫자)..(숫자) 을 이용하는데 표현식에서 숫자가 없으면 끝까지 참조하는 것을 의미함.
/// - ` .. ` -> 시작부터 끝까지
/// - `.. 숫자` -> 시작부터 숫자 미만까지
/// - `숫자 ..` -> 숫자부터 끝까지
/// - `숫자 .. 숫자` -> 숫자부터 숫자 미만까지
///
///  참조자인 슬라이스를 사용하기 때문에 불변성 관련된 검사를 컴파일러가 해줌.
///
fn using_string_slice() {
    let sentance = String::from("안녕하세요 감자 서버에요");
    #[allow(unused_mut)]
    let mut mut_sentance = String::from("서버를 닫습니다");
    // utf8 에서 한글은 대부분 3바이트
    let first_gylph = &sentance[..3];
    let second_gylph = &sentance[3..6];
    // 이 밑에 어떻게 처리할까 하면 crate이 아니라 C++, 다른 라이브러리 문서를 봐야 겠지.
    println! {"{:x?} {}",second_gylph.as_bytes(),first_gylph};

    // 문자열의 첫 번째 단어를 반환하는 함수
    // 문자열의 일부 -> 슬라이스로 반환
    fn trim_first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[i + 1..];
            }
        }
        &s[..]
    }

    println!("첫 단어 : {}", trim_first_word(&sentance));

    let less = trim_first_word(&mut_sentance);
    // mut_sentance.clear();

    println!("{}", less);
}
///
/// ## 슬라이스 관련 상호작용
///
/// - 문자열 리터럴 -> 쓸 때보면 참조자, 불변이면서 슬라이스이기도 함.
/// - 매개변수, 반환값도 일반적인 슬라이스를 사용하게 하면 사용하기 쉽다.
///
fn slice_related_action() {
    #[allow(unused_variables)]
    let s = "이건 불변 참조자인 문자열 슬라이스입니다.";

    // &String 대신 &str
    fn trim_first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[i + 1..];
            }
        }
        s
    }
    let my_string = String::from("이건 String입니다.");

    // `first_word`는 `String`의 일부 혹은 전체 슬라이스에 대해 작동합니다
    //
    let word = trim_first_word(&my_string[0..6]);
    println!("{}", word);
    let word = trim_first_word(&my_string[..]);
    println!("{}", word);

    // 또한 `first_word`는 `String`의 전체 슬라이스와 동일한 `String`의
    // 참조자에 대해서도 작동합니다
    //
    let word = trim_first_word(&my_string);
    println!("{}", word);

    let my_string_literal = "hello world";

    //
    // `first_word`는 문자열 리터럴의 일부 혹은 전체 슬라이스에 대해 작동합니다
    //
    let word = trim_first_word(&my_string_literal[0..6]);
    println!("{}", word);
    #[allow(clippy::redundant_slicing)] // 예시로 전체 범위를 슬라이스 문법으로 표시.
    let word = trim_first_word(&my_string_literal[..]);
    println!("{}", word);

    // 문자열 리터럴은 *곧* 문자열 슬라이스이므로,
    // 아래의 코드도 슬라이스 문법 없이 작동합니다!
    let word = trim_first_word(my_string_literal);
    println!("{}", word);
}
