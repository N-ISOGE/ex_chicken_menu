//!
//! # 구조체란
//!
//! - 구조체
//! - 필드 초기화 축약법 및 편리한 기능
//! - 구조체 비슷하면서 편리한 것들
//! - 구조체 데이터의 소유권
//!

#[allow(unused)]
pub fn main() {
    println!("슬라이스 the_slice -> 구조체란? the_defining_structs");
    // - 구조체
    defining_structs();
    // - 필드 초기화 축약법 및 편리한 기능
    example_of_field_init_shorthand();
    // - 구조체 비슷하면서 편리한 것들
    example_of_struct_like();
    // - 구조체 데이터의 소유권
    lifetime_in_instance();
    println!("구조체란? the_defining_structs -> 구조체를 사용한 예시 the_example_of_using_structs");
}

///
/// ## 구조체
///
/// - `struct` 키워드 사용
/// - 구성요소 field라고 부름
/// - 구조체 정의에 따라 생성한 것을 instance로 부름
/// - `mut`에 따라 불변, 가변 가능
///
/// 예시
///
fn defining_structs() {
    struct SoDog {
        wow: bool,
        so: String,
        many: i32,
    }
    // 불변
    let wow = SoDog {
        wow: true,
        so: String::from("cute"),
        many: 3000,
    };
    // 가변
    let mut much_wow = SoDog {
        wow: true,
        so: String::from("cute"),
        many: 3000,
    };
    much_wow.so = String::from("so so so cute");
    println!("immut {} {} {}", wow.wow, wow.so, wow.many);
    println!("mut {} {} {}", much_wow.wow, much_wow.so, much_wow.many);
}

///
/// ## 필드 초기화 축약법 및 편리한 기능
/// - 변수명과 구조체 필드명이 같으면 생략 가능
/// - 기존의 인스턴스를 이용해 새 인스턴스를 만들 때, 기존의 값을 재활용 가능
///     - 기존의 인스턴스는 복사 트레이트가 없는 값이면 이동되어 사용 불가.
///
/// 예시
///
fn example_of_field_init_shorthand() {
    struct SoDog {
        wow: bool,
        so: String,
        many: i32,
    }

    // so, many를 생략함
    fn init_so_wow_dog(so: String, many: i32) -> SoDog {
        SoDog {
            wow: true,
            so,
            many,
        }
    }

    let so_wow_dog = init_so_wow_dog(String::from("cute"), 3000);

    // 명시한 wow값을 제외한 나머지 값을 so_wow_dog의 값을 재활용
    let not_so_wow_dog = SoDog {
        wow: false,
        ..so_wow_dog
    };
    // so_wow_dog값을 이동시켜 초기화하기 때문에 사용 불가
    // println!("{}",so_wow_dog.so);
    println!("{} {} {}", not_so_wow_dog.so, not_so_wow_dog.wow, not_so_wow_dog.many);
}

///
/// ## 구조체 비슷하면서 편리한 것들
///
/// - 필드명이 크게 필요없는 직관적인 자료형이 필요한 경우
///     - 튜플에 이름 붙인 튜플 구조체 사용 가능
/// - 아님 필드가 없는 구조체 사용 가능
///     - 유사 유닛 구조체라고 부름
///     - 어떤 타입에 대해 트레이트를 구현하고 싶지만 타입 내부에 어떤 데이터를 저장할 필요는 없을 경우 유용합니다.
///
/// 예시
///
fn example_of_struct_like() {
    // 튜플에 이름 붙인 튜플 구조체 사용 가능
    struct Point(i32, i32);
    let origin = Point(0, 0);

    // 필드가 없는 구조체, 유닛 구조체
    struct Off;
    #[allow(unused_variables)]
    let sign = Off;

    println!("{:?}", origin.0);
}

///
/// ## 구조체 데이터의 소유권
///
/// 구조체의 인스턴스가 소유권을 가지지 않는 데이터를 저장할 때는
/// 인스턴스가 유효한 동안 인스턴스 내의 모든 데이터가 유효함을 보장해야 함.
///
fn lifetime_in_instance() {
    #[allow(unused)]
    struct User {
        active: bool,
        // 보장 못 함
        // username: &str,
        // email: &str,
        sign_in_count: u64,
    }

    #[allow(unused_variables)]
    let user1 = User {
        active: true,
        // username: "someusername123",
        // email: "someone@example.com",
        sign_in_count: 1,
    };
}
