//! # 콜렉션에 나온 예제들
//!
//! ## 중간값, 최빈값
//! - [`median_and_mode`]
//! 정수 리스트가 주어졌을 때,
//! - 벡터를 이용하여 이 리스트의 중간값 (median, 정렬했을 때 가장 가운데 위치한 값), 그리고
//! - 최빈값 (mode, 가장 많이 발생한 값; 해시맵이 여기서 도움이 될 것입니다) 을 반환해 보세요.
//!
//! ## pig latin
//! 문자열을 피그 라틴 (pig Latin) 으로 변경해 보세요.
//!
//! 각 단어의 첫 번째 자음은 단어의 끝으로 이동하고 ‘ay’를 붙이므로,
//! - ‘first’는 ‘irst-fay’가 됩니다.
//!
//! 모음으로 시작하는 단어는 대신 끝에 ‘hay’를 붙입니다.
//! - (‘apple’은 ‘apple-hay’가 됩니다.)
//!
//! UTF-8 인코딩에 대한 세부 사항을 명심하세요!
//!
//! ## 사원 목록
//! 해시맵과 벡터를 이용하여 사용자가 회사 부서의 직원 이름을 추가할 수 있도록 하는
//! 텍스트 인터페이스를 만들어 보세요.
//!
//! 예를 들어 ‘Add Sally to Engineering’이나 ‘Add Amir to Sales’ 같은 식으로요.
//! 그 후 사용자가 모든 사람에 대해 알파벳 순으로 정렬된 목록이나
//! 부서별 모든 사람에 대한 목록을 조회할 수 있도록 해보세요.


pub fn median_and_mode(numbers: &Vec<i64>) -> (i64, i64) {
    // 숫자 리스트에서 중간값, 최빈값을 찾기.
    use std::collections::HashMap;

    let mut num_map: HashMap<i64, usize> = HashMap::new();

    for number in numbers {
        let count = num_map.entry(*number).or_default();
        *count += 1;
    }

    let mode_num = *(num_map.iter().max_by_key(|k_v| k_v.1)).unwrap().0;

    // 숫자의 개수
    let mut key_vec: Vec<i64> = num_map.keys().copied().collect();

    key_vec.sort_unstable();

    //
    let number_len = numbers.len();
    let mut stack_number: usize = Default::default();
    let mut selected: i64 = Default::default();

    for key in key_vec {
        match num_map.get(&key) {
            None => {
                panic!("unexpected key");
            }
            Some(count) => {
                stack_number += count;
            }
        }
        if number_len < stack_number * 2 {
            selected = key;
            break;
        }
    }

    let median_num = selected;

    (mode_num, median_num)
}

#[test]
fn test_mode_med_0() {
    let number: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1];

    let (mode, med) = median_and_mode(&number);

    debug_assert_eq!(mode, 0);
    debug_assert_eq!(med, 0);
}

#[test]
fn test_mode_med_1_10() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9];

    let (mode, med) = median_and_mode(&numbers);

    debug_assert_eq!(med, 6);
    debug_assert_eq!(mode, 9);
}

pub fn pig_latin(input: &str) -> String {
    let eng_vowel: &'static str = "aeiouAEIOU";

    input
        .split_whitespace()
        .map(|word| -> String {
            let first_letter = match word.chars().nth(0) {
                Some(glyph) => glyph,
                None => {
                    panic!("invalid split result: whitespace");
                }
            };

            if eng_vowel.contains(first_letter) {
                format!("{word}-hay").to_string()
            } else {
                let headless_word: String = word.chars().skip(1).collect();
                format!("{headless_word}-{first_letter}ay").to_string()
            }
        })
        .fold(String::new(), |acc, x| format!("{} {x}", acc.trim()))
}

#[test]
fn test_pig_latin_table() {
    let word = "wooden table in cabin";
    let expected = "ooden-way able-tay in-hay abin-cay";
    let result = pig_latin(word);

    debug_assert_eq!(expected, result);
}

#[test]
fn test_pig_latin_empty() {
    let word = " \t\n";
    let expected = "";
    let result = pig_latin(word);

    debug_assert_eq!(expected, result);
}
