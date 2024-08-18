//! ## 패키지와 크레이트
//!
//! - [`WhatIsCrate`]
//! - [`WhatIsPackage`]
//! - [`ExamplePackage`]
//! - [`ModuleOverview`]
//! - [`example_mod_by_front`]
//! - [`example_of_using_path`]
//! - [`MainAndLibsBoth`]
//! - [`example_of_relative_path`]
//! - [`struct_enum_control_permission`]
//! - [`example_of_use`]
//!

/// # 크레이트란? 용어 위주로
///
/// ## 크레이트 :
/// - 러스트가 컴파일할 때 한 번 고려하는 가장 작은 코드 단위
/// - 컴파일러한테 단일 소스 파일을 줘도 크레이트라고 여김.
///
/// ## 크레이트 내용:  
/// - 여러 모듈이 담겨 있고,
/// - 다른 곳에 있는 모듈도 정의되어 있을 수 있다.
///
/// ## 크레이트 구분 중 하나: 바이너리, 라이브러리  
/// - 바이너리 크레이트 : 실행파일로 컴파일 가능 -> 시작점, main() 필요
/// - 라이브러리 크레이트 : 실행파일로 컴파일되지 않음.
///
/// ## 크레이트 루트  
/// 컴파일러가 컴파일을 시작하는 소스파일.
/// 크레이트의 루트 모듈을 구성함.
///
#[allow(dead_code)]
struct WhatIsCrate;

/// ## 패키지?
///
/// ### 패키지 : 일런의 기능을 제공하는 하나 이상의 크레이트로 구성된 번들
///
/// 패키지에 있는 `Cargo.toml`: 크레이트를 빌드하는 방법을 설명함.  
/// -> Cargo, 커맨드 라인 빌드 도구의 바이너리 크레이트가 포함된 패키지  
/// 카고가 의존성이나 빌드 자체를 `Cargo.toml` 읽고 처리해줌
///
/// ### 제한 사항
/// - 여러 개의 바이너리 크레이트 가능
/// - 라이브러리 크레이트는 하나만 가능
#[allow(dead_code)]
struct WhatIsPackage;

/// ## 패키지 만드는 예시
///
/// 패키지를 정의할 때, 명시적으로 크레이트 루트를 정하지 않으면  
/// 카고는 `src/main.rs`, `src/lib.rs`을 찾고 사용함.  
/// 한 패키지에 두 파일은 각각 같은 이름의 바이너리, 라이브러리를 나타냄.  
///
#[allow(dead_code)]
struct ExamplePackage;

/// ## 모듈 : 스코프, 공개 범위 제어  
/// > 키워드: 모듈, 경로, `use`, `pub`, `as`, 외부 패키지, glob 연산자
///
/// ### 모듈 치트 시트
///
/// - 크레이트 루트부터 시작 : 컴파일러가 먼저 보는 것
/// - 모듈 선언 : 루트 파일에서 모듈 선언, 아래의 목록에서 선언한 모듈의 코드 찾음
///     - `mod <module_name>`에서 세미콜론 대신 중괄호면 중괄호 안, 인라인 코드
///     - src/<module_name>.rs
///     - src/<module_name>/mod.rs
/// - 서브모듈 선언 : 모듈 안에서 모듈을 선언 가능, 모듈 찾는 방법은 위와 동일하게 적용
/// - 모듈 내 코드로의 경로 : 모듈이 공개 규칙에 허용하는 한도 내에서 경로에 따라 참조 가능
///     - `crate::<module_name>::...::<deference_name>`
/// - 기본적으로는 비공개: 공개하고 싶은 모듈, 아이템 앞에 `pub`를 붙여야 함.
/// - use 키워드: 단축경로, 같은 스코프에 있듯이 이름을 짧게 줄여 쓸 수 있음.
///
#[allow(dead_code)]
struct ModuleOverview;

/// ## 모듈 예시, 흩어진 코드 모아서 구조화
/// 예시로 모듈 안에 인라인으로 모듈 정의함.
///
/// 모듈 트리, 크레이트 모듈 구조에서 최상위는 `crate`
/// - 기본적으론 여기서부터 시작함.
/// 트리 구조에서 사용하던 단어 sibling, parent, child 는 동일하게 사용
#[allow(dead_code)]
mod example_mod_by_front {
    mod preparing {
        fn swap_knife() {}
    }

    pub mod hosting {
        fn add_to_waitlist() {}
        // ...
    }
    pub mod serving {
        pub fn take_order() {}
        // ...
    }
}

/// ## 경로 : 경로를 사용해서 모듈 트리의 아이템 참조하기
///
/// 경로 종류
/// - 절대 경로: 내부는 `crate`, 크레이트 루트, 외부는 해당 크레이트 이름으로 부터 시작.
/// - 상대 경로: 현재 모듈을 기점으로 함. `self`, `super`, 현재 모듈 내 식별자 사용
///
/// 경로 종류를 선택하는데 고려할 점
/// - 상대경로는 사용하는 모듈 상대적인 위치 변화에 따라 경로가 맞지 않게 되는 경우가 발생함.
/// - 절대 경로는 사용되는 모듈의 위치 변화에 따라 경로가 맞지 않게 되는 경우가 발생함.
///
/// 경로에 따라 참조하는 경우, 공개 규칙 예시
#[allow(dead_code)]
fn example_of_using_path() {
    // 비공개된 모듈 접근
    // example_mod_by_front::preparing::swap_knife();
    // 모듈은 공개되어 있지만 함수가 공개되어 있지 않은 상태로 접근
    // example_mod_by_front::hosting::add_to_waitlist();
    // 공개된 모듈의 공개된 함수 접근
    example_mod_by_front::serving::take_order();
    // 절대 경로
    crate::rust_books::the_package_and_crate::example_mod_by_front::serving::take_order();
}

/// ## 바이너리, 라이브러리 크레이트를 동시에 가지는 경우
///
/// 모듈 트리를 라이브러리 크레이트에서 정의해서 바이너리 크레이트에서 사용하기
/// 다른 사람이나 같은 패키지에서나 동일하게 접근 가능해짐
#[allow(dead_code)]
struct MainAndLibsBoth;

/// ## 상대 경로
/// 경로를 `super`로 시작하면 부모 모듈로부터 시작함.
/// 부모 모듈에 참조할 아이템이 동일한 상황 등에서 편리.
#[allow(dead_code)]
mod example_of_relative_path {
    fn pop() {}
    mod wow {
        /// 부모 모듈에 있는 pop()를 호출
        fn party() {
            super::pop();
        }
    }
}

/// ## 구조체도 내부도 기본은 비공개, 열거형은 내부도 같이 공개
///
/// `pub`으로 구조체 공개해도 내부 필드나 함수는 기본적으로 비공개.
/// 공개한다고 해야만 공개됨.
///
/// 따라서 `pub`를 사용하지 않은 필드가 있다면 그 비공개 필드를 초기화하는 생성 함수가 필요
///
/// 열거형은 열거형이 공개되면 내부도 전부 공개
/// - 모든 경우의 수 중 하나 -> 따로 관리 하기 힘듬.
#[allow(dead_code)]
pub mod struct_enum_control_permission {
    pub mod wallet {
        pub struct Card {
            pub name: String,
            pub number: [i16; 4],
            cvc_code: i16,
        }
        impl Card {
            pub fn init_example_card(name: &str) -> Card {
                Card {
                    name: String::from(name),
                    number: [123, 456, 789, 0],
                    cvc_code: 000,
                }
            }
        }
    }

    pub fn test_making_card() {
        let mut sample = wallet::Card::init_example_card("aaaa");
        sample.name = String::from("bbb");
        println!("{} is owner", sample.name);

        // 공개되지 않은 필드를 사용하는 함수 호출 -> 에러
        // sample.cvc_code = 1234;
    }
}

/// # `use`로 경로를 스코프 안으로 가져오기, `as`로 별명 지어주기
///
/// 같은 스코프에서만 use로 선언한 단축경로가 유효함.
/// 기존 모듈 공개 규칙에는 영향을 받음. -> 공개된 모듈만 가능.
///
/// use로 경로를 단축할 때 어느 부모 모듈에서 왔는지 알 수 있게 부모 모듈까지만 use 사용한 뒤 함수 호출시에는 부모 모듈을 밝힘.
///
/// ### `as`로 단축경로에 대한 별명을 선언할 수 있음.
///
/// ### `pub use`, 단축 경로 재공개
///
/// `use`도 기본적으로 비공개, `pub`을 사용하면 다른 곳에서도 단축 경로 사용 가능.
///
fn example_of_use() {
    pub use struct_enum_control_permission::wallet as example_wallet;

    let some_card = example_wallet::Card::init_example_card("hello");

    println!("{}'s card : {:?}", some_card.name, some_card.number);
}

#[allow(unused)]
pub fn main() {
    struct_enum_control_permission::test_making_card();
    example_of_use();
}
