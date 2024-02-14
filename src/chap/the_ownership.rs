///
/// # 소유권
///
/// - [`소유권`]
/// - [`소유권 규칙`]
/// - [`변수의 스코프`]
/// - [`String 타입으로 예시를 듬`]
/// - [`이어지는 메모리와 할당`]
/// - [`소유권과 함수`]
/// - [`반환값과 스코프`]
///
#[allow(unused)]
pub fn main() {
    // # 개요
    println!("변수와 가변성 the_mutability_shadowing -> 소유권 the_ownership");
    // - [`소유권`]
    // - [`소유권 규칙`]
    // - [`변수의 스코프`]
    // - [`String 타입으로 예시를 듬`]
    using_heap();
    // - [`이어지는 메모리와 할당`]
    scope_based_memory_management();
    differance_stack_heap();
    move_ownership();
    use_clone();
    // `Copy`
    // - [`소유권과 함수`]
    give_value_to_function();
    // - [`반환값과 스코프`]
    return_value_and_ownership();

    // 함수에 값을 전달하되 소유권을 전달하지 않는 방법이 있나?
    println!("소유권 chap_ownershuip -> 참조자 chap_referance");
}

/// ## `소유권`
/// - 소유권은 러스트 프로그램의 메모리 관리법을 지배하는 규칙 모음
/// - 컴파일러가 컴파일 중에 이런 규칙에 따라 검사하는 것으로 메모리를 관리하게 만듬
///     - 다른 메모리 관리 예시 : 가비지 콜렉션; 명시적으로 할당, 해제; 생성자, 소멸자
///     - 프로그래머가 전부 책임지고 할당해제할 때 놓치는 것
///     - 실행 중에 프로그램이 메모리를 할당해제하는 것으로 속도가 느려지는 것

/// ## `소유권 규칙`
/// - 요약
///     - 러스트에서 각각의 값은 소유자가 정해져 있습니다.
///     - 한 값의 소유자는 동시에 여럿 존재할 수 없습니다.
///     - 소유자가 스코프 밖으로 벗어날 때, 값은 버려집니다. (dropped)

/// ## `변수의 스코프`
/// 아직은 다른 언어와 큰 차이없이 스코프 내에서 유효함.

/// ## `String 타입으로 예시를 듬`
/// - 힙에 저장되면서 러스트의 데이터 정리 과정을 보여주기 위해 String 타입을 사용
/// - String 타입 특징으로 힙을 사용함을 꼽음
///
fn using_heap() {
    fn calc_len(s: &str) -> usize {
        // &String : String에 대한 참조자
        s.len()
    }
    // mut, String 타입 생성, :: -> 네임스페이스 연산자
    let mut s1 = String::from("hello");

    println!("before : {}", s1);

    // 변경 가능, 문자열 리터럴 추가
    s1.push_str(", 世계!");

    println!("after : {}", s1);

    // & 연산자 사용, 참조자
    println!("hello : {}", calc_len(&s1));
    println!("no problem to print {}", s1);
}

/// # `이어지는 메모리와 할당`
/// - 문자열 리터럴은 컴파일할 때 내용을 알 수 있다
///     - 최종 실행파일에 하드코딩됨
/// - String 타입은 컴파일할 때 내용을 몰라도 된다
///     - 힙에 메모리를 할당하는 방식으로 텍스트 내용과 크기가 변경 가능
///
/// String 타입을 이용하는 과정에서 필요한 것
/// 1. 실행 중 메모리 할당자로부터 메모리를 요청해야 합니다.
/// 2. String 사용을 마쳤을 때 메모리를 해제할 방법이 필요합니다.
///     - 메모리 해제, 할당자에게 메모리를 반납
///
/// 1. `String::from`호출할 때, 필요한 만큼 요청함
/// 2. 메모리 해제하는 것이 문제
///     - 가비지 컬렉터나 직접 할당 및 해제하는 방법은 단점이 있다
///     - 러스트는?         

/// ## 러스트 방식
/// 스코프를 벗어나는 순간 메모리를 해제하는 방식
/// - 지역 변수 쓰는 방식과 같지 않나?
/// - 정적 검사를 사용하여 실수를 방지, 문제있으면 컴파일러가 알려줌.
///
/// - 스코프 밖으로 나가면 `drop` 함수 호출
///     - 소멸자와 같은 역할, RAII
#[allow(unused_variables)]
fn scope_based_memory_management() {
    // 이 스코프에서는 유효함
    let s = String::from("hello");
} // 스코프가 종료되고 s는 유효하지 않다.

/// ### 변수와 데이터 간 상호작용 방식
/// #### 이동
#[allow(unused_variables)]
fn differance_stack_heap() {
    // 스택을 사용하는 경우, 단순히 스택에 저장하고 복사됨
    let x = 5;
    let y = x;
    // 하지만 힙을 사용하는 경우는 다르다.
    let s1 = String::from("hello");
    let s2 = s1;
}

/// 이때 s1, s2의 스택에 저장한 값, 주소값만 변경되지 힙에 있는 값들은 변경되지 않음
/// - 깊은 복사가 아님, 근데
///
/// 근데 같은 주소를 가리키는 상황을 생각해볼 때
/// - 두 변수를 통해 동시에 메모리 해제가 되어 이중 해제되면 문제가 발생함
///
/// 그래서 러스트에서는 하나를 유효하지 않다고 해버림
fn move_ownership() {
    let s1 = String::from("빌린 ");
    // 아래줄 주석해제할 경우, 이동한 값을 사용했다고 오류가 뜸, s1이 유효하지 않게 됨
    // let s2 = s1;
    println!("{}, 문자열?", s1);
}

/// 러스트에서는 이것을 이동이라고 함, 얕은 복사가 아니라
/// - 주소; 포인터, 길이, 용량 값을 복사하니 얕은 복사인 것처럼 보임
/// - 근데 기존의 변수를 무효화하는 것을 포함해서 이동으로 부름.
///
/// #### 클론
/// 깊은 복사를 할 때는 `clone`이라는 공용 메서드를 사용
fn use_clone() {
    let s1 = String::from("clone임");
    // 이 클론을 사용하는 것은 명시적으로 성능에 영향을 줄 수 있다는 표시의 기능을 함
    let s2 = s1.clone();

    println!("s1 {} s2 {}", s1, s2);
}
/// #### 복사
/// `Copy` 트레이트를 가지고 있는 타입일 경우, 스택에서 저장 복사가 됨.
/// 하지만 `Drop` 트레이트가 구현되어 있을 경우, `Copy` 트레이트를 어노테이션 불가함.
/// - 타입이 복사가능한지 안 하는지 컴파일러에게 알려 주는 방식.
/// 예시 중 특이한 걸로는 `Copy` 가능한 타입만으로 구성된 튜플이 있다.

/// # `소유권과 함수`
/// 함수에 값을 전달하는 경우
fn give_value_to_function() {
    // s가 스코프 안에서 생성됨
    let s = String::from("소유권과 함수");
    // s가 함수로 이동됨, 이후에 s는 유효하지 않음
    takes_ownership(s);

    // x가 스코프 안에서 생성됨
    let x = 5;
    // x가 함수로 이동됨, 하지만 i32 is `Copy` 따라서 이후에도 x는 유효함
    makes_copy(x);
} // x, s 둘 다 벗어나지만 s가 이동해서 이미 유효하지 않음으로 메모리 해제같은 다른 작업이 없음.

fn takes_ownership(some_string: String) {
    // some_string이 스코프 안으로 들어옴.
    println!("take ownership : {}", some_string);
} // 여기서 some_string이 스코프에서 벗어나고 `drop` 호출, 메모리 해제.

fn makes_copy(some_integer: i32) {
    // some_integer가 스코프 안으로 들어옴.
    println!("make copy : {}", some_integer);
} // 여기서 some_integer가 스코프에서 벗어나도 기존 변수에 영향이 없음.

/// # `반환 값과 스코프`
/// 값을 반환하는 과정에서의 소유권 취급
///
fn return_value_and_ownership() {
    // `gives_ownership()`이 반환한 값을 `s1`에 대입
    let s1 = gives_ownership();

    // s2 생성됨
    let s2 = String::from("받을 값");

    // s2가 함수로 이동된 뒤, 함수가 반환한 값을 s3에 이동함.
    let s3 = takes_and_gives_back(s2);

    // s2는 이미 이동되어 유효하지 않음
    println!("s1 : {}, s2! , s3 : {}", s1, /* s2, */ s3);
} // s1, s3는 스코프 밖으로 벗어나면서 `drop` 호출, 메모리 해제
  // s2는 이동되어 유효하지 않아 영향이 없음

#[allow(clippy::let_and_return)]
fn gives_ownership() -> String {
    // 반환 값을 호출한 곳으로 이동시킴.
    // some_string이 생성됨
    let some_string = String::from("줄 값");
    // some_string이 호출한 곳으로 이동됨
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string이 함수로 이동됨.
    // a_string이 반환, 호출한 곳으로 이동됨.
    a_string
}
