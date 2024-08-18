//! # Result, 복구 가능한 에러 처리
//!
//! ## 개요
//!
//! 에러가 났지만 대응할 여지가 있다면 중단하지 않을 수 있다.
//! - 파일이 없어서 에러가 났다면 종료하는 대신 파일을 만드는 것을 고려할 수 있다.
//!
//! Result 열거형 모양
//! - [`signature_of_result`]
//! ```rust
//! enum Result<T, E> {
//!     Ok(T),
//!     Err(E),
//! }
//! ```
//!
//!
//! ## Result 사용해보기
//!
//! - [`test_using_function_with_result`]
//!
//! ## 여러 에러에 대해 매칭하기
//!
//! - [`test_matching_each_errors`]
//!
//! - [`test_using_closures_for_error`]
//!
//!
//! ## unwrap, expect, 패닉 다루기
//!
//! - [`test_using_unwrap`]
//!
//! ## 에러 전파하기
//!
//! 에러에 대한 정보를 호출하는 곳에서 이해할 수 있게 반환하여
//! 반환한 정보를 가지고 대처하도록 함.
//! - [`test_propagating_error`]
//! - [`read_username_file_result`]
//!
//! ## ? 연산자
//!
//! `?` 연산자 ->
//! `from` 함수 ->
//! 각각의 에러에 대해 구현한 `from`함수가 자체 구현한 에러로 변환해줌.
//!
//! ```rust
//! use std::{fs::File,io::Read};
//! fn read_string_from_file() -> Result<String,std::io::Error>{
//! let input_path = "tests/username.txt";
//! let mut username = String::new();
//!   File::open(input_path)?.read_to_string(&mut username)?;
//!   Ok(username)
//! }
//! ```
//! - [`read_username_from_file`]
//! - [`test_question_mark_for_result`]
//!
//! ## ? 사용 할만한 곳
//!
//! `?`은 match 표현식에서 사용한 방식같이 변환가능한 `Result`로 바꿔서 전달하는 일을 함.
//! 변환할 수 없다면 `?`이 변환할 수 없다는 에러메세지가 뜸.
//!
//! ```rust
//!
//! # fn what() -> Result<(),std::io::Error> {
//!     // 반환 형식이 () -> ?, from 으로 변환 불가
//!     let file = std::fs::File::open("wow")?;
//! # Ok(())
//! # }
//! ```
//!
//! > error[E0277]:
//! > the `?` operator can only be used in a function
//! > that returns `Result` or `Option`
//! > (or another type that implements `FromResidual`)
//!
//! 1. `result, Option` 이거나 `FromResidual`을 구현해야 함.
//! 2. 반환형식에서 받는 타입으로 반환하게 수정하거나
//! 3. Result을 대응하기 위해 match 표현식, Result의 메서드 사용.
//!
//! 더불어 **`Option`에도 `?`을 사용**, 의도한 타입이 아니면 에러처럼 `None` 선택
//!
//! - [`last_char_of_first_line`]
//!
//! `Option`, `Result` 각각의 반환값을 변환할 때 `?`나 암시적으로 변환 불가.
//! **명시적으로 `Option`, `Result` 반환값을 변환해야 함.**
//!
//!
//! **main은 결과를 `()`로 반환하는 형태, 그러면 `Result<(), E>와 비슷하고 가능함.**
//!
//! - [`test_use_result`]
//! - [`test_result_return_error`]
//!

use std::path;
use test_log::test;

#[allow(dead_code)]
static TEST_DIRECTORY_NAME: &'static str = "tests";

#[test]
fn signature_of_result() {
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    use std::fmt;

    impl fmt::Display for Result<i32, String> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Result::Ok(num) => {
                    write!(f, "{}", num)
                }
                Result::Err(error_thing) => {
                    write!(f, "wow error {}", error_thing)
                }
            }
        }
    }

    debug_assert_eq!("1", format!("{}", Result::Ok(1)));
    debug_assert_eq!(
        "wow error wow",
        format!("{}", Result::Err(String::from("wow")))
    );
}

#[test]
#[should_panic(expected = "파일 여는 도중")]
fn test_using_function_with_result() {
    use std::{fs, io, path};

    let result_path = path::Path::new(TEST_DIRECTORY_NAME).join("function_with_result.txt");

    if let Ok(_) = fs::File::open(&result_path) {
        if let Err(error) = fs::remove_file(&result_path) {
            if error.kind() != io::ErrorKind::NotFound {
                panic!("!> 파일 여는 도중 삭제 실패: {:?}", error);
            }
        }
    }

    // Result 반환 -> 특정 타입 Result<fs::File, io::Error>
    // 결과를 알려주는 방법으로 성공, 실패를 통합한 결과 열거형 Result 반환.
    let greeting_file_result = fs::File::open(&result_path);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("!> 파일 여는 도중 문제 발생: {:?}", error),
    };

    debug_assert!(greeting_file.metadata().unwrap().is_file());
}

#[test]
fn test_matching_each_errors() {
    use std::{fs, io::ErrorKind, path};

    let result_path = path::Path::new(TEST_DIRECTORY_NAME).join("matching_each_errors.txt");

    if let Err(error) = fs::create_dir(TEST_DIRECTORY_NAME) {
        match error.kind() {
            ErrorKind::AlreadyExists => {}
            _other_error => return,
        }
    }

    if let Ok(_) = fs::File::open(&result_path) {
        if let Err(error) = fs::remove_file(&result_path) {
            if error.kind() != ErrorKind::NotFound {
                panic!("!> 파일 여는 도중 삭제 실패: {:?}", error);
            }
        }
    }

    let greeting_file_result = fs::File::open(&result_path);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match fs::File::create(&result_path) {
                Ok(file_created) => file_created,
                Err(error_while_creating) => {
                    panic!("!> 파일 생성 도중 발생한 문제: {:?}", error_while_creating)
                }
            },
            other_error => {
                panic!("!> 파일 여는 도중 발생한 문제: {:?}", other_error)
            }
        },
    };

    debug_assert!(greeting_file.metadata().unwrap().is_file());
}

#[test]
fn test_using_closures_for_error() {
    use std::{fs, io::ErrorKind};

    let result_path = path::Path::new(TEST_DIRECTORY_NAME).join("closures_for_error.txt");

    if let Err(error) = fs::create_dir(TEST_DIRECTORY_NAME) {
        match error.kind() {
            ErrorKind::AlreadyExists => {}
            _other_error => return,
        }
    }

    if let Ok(_) = fs::File::open(&result_path) {
        if let Err(error) = fs::remove_file(&result_path) {
            if error.kind() != ErrorKind::NotFound {
                panic!("!> 파일 여는 도중 삭제 실패: {:?}", error);
            }
        }
    }

    let greeting_file = fs::File::open(&result_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            fs::File::create(&result_path)
                .unwrap_or_else(|error| panic!("!> 파일 생성 도중 발생한 문제: {:?}", error))
        } else {
            panic!("!> 파일 여는 도중 발생한 문제: {:?}", error)
        }
    });

    debug_assert!(greeting_file.metadata().unwrap().is_file());
}

#[test]
#[should_panic]
fn test_using_unwrap() {
    use std::{fs, io};

    let result_path = path::Path::new(TEST_DIRECTORY_NAME).join("using_unwrap.txt");

    if let Ok(_) = fs::File::open(&result_path) {
        if let Err(error) = fs::remove_file(&result_path) {
            if error.kind() != io::ErrorKind::NotFound {
                panic!("!> 파일 여는 도중 삭제 실패: {:?}", error);
            }
        }
    }

    // Ok(T)에서 T를 반환하거나 Err(E)에서 panic!에 에러 메세지를 전달하여 출력하게 함.
    let _greeting_file = fs::File::open(&result_path).expect("파일이 없거나 엑세스가 거부됨");

    // Ok(T)에서 T를 반환하거나 Err(E)에서 panic!를 호출함.
    let _greeting_file = fs::File::open(&result_path).unwrap();
}

#[allow(dead_code)]
fn read_username_file_result(input_path: &path::Path) -> Result<String, std::io::Error> {
    // 반환 타입 Result<T,E>에서 String, std::io::Error로 정해지면서
    // 성공하면 String 담은 OK값, 문제일 경우 문제의 정보를 담은 io::Error를 담은 Err값 반환
    // io::Error를 사용한 이유는 내부에서 사용하는 File::open(), read_to_string()이 같은 형식의 에러를 반환함.
    //
    use std::{fs, io::Read};

    // Result로 결과를 반환받고
    let username_file_result = fs::File::open(input_path);

    let mut username_file = match username_file_result {
        // 파일을 여는데 성공했다면 0k형식으로 파일을 전달받음.
        Ok(file) => file,
        // 문제가 생기면 에러를 반환하며 함수를 종료함.
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // 여기서 파일을 읽을 때 결과를 match 표현식에 넘김.
    match username_file.read_to_string(&mut username) {
        // 성공했다면 읽은 결과인 username을 OK로 표현해서 넘김.
        Ok(_) => Ok(username),
        // 실패했다면 에러를 반환함.
        Err(e) => Err(e),
    }
}

#[test]
fn test_propagating_error() {
    use std::{io, path};
    let some_path = path::Path::new(TEST_DIRECTORY_NAME).join("username.txt");

    if let Err(error) = read_username_file_result(&some_path) {
        debug_assert_eq!(io::ErrorKind::NotFound, error.kind());
    }
}

pub fn read_username_from_file(input_path: &path::Path) -> Result<String, std::io::Error> {
    use std::fs;
    // `?`연산자는 `from` 함수를 거침.
    // `from`은 형식을 변환하는 함수로 여기선 반환하는 타입에 맞춰 에러를 전파하는 역할함.
    fs::read_to_string(input_path)
}

#[test]
fn test_question_mark_for_result() {
    use std::{fs, io::Write, path};

    let test_path = path::Path::new(TEST_DIRECTORY_NAME).join("question_mark.txt");

    match fs::File::create_new(&test_path) {
        Ok(mut file) => {
            let _ = file.write_fmt(format_args!("{}", "hello"));
        }
        Err(_) => {}
    }

    if let Ok(name) = read_username_from_file(&test_path) {
        debug_assert_eq!(name, "hello");
    }
}

pub fn last_char_of_first_line(text: &str) -> Option<char> {
    // 결과로 문자가 있을지 없을지 모르기 때문에 Option 사용
    // 첫 줄이 있는지 확인하고 없으면 `None` 반환
    text.lines()
        .next()?
        // 첫 줄이 있고 마지막을 받는데 여기서도 `Option` 반환
        .chars()
        .last()
}

#[test]
fn test_use_result() -> Result<(), String> {
    Ok(())
    // Result<(),E> 반환할 시 should_panic 어노테이션 사용 불가
    // Err(String::from("I am error"))
}

#[test]
fn test_result_return_error() -> Result<(), Box<dyn std::error::Error>> {
    let _file = std::fs::File::open(TEST_DIRECTORY_NAME);
    Ok(())
}
