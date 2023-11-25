참조자와 빌림, reference and borrowing
===================================

참조자
-----

**값의 소유권을 넘기는 대신** 개체에 대한 참조자를 사용할 수 있다 
```rust
fn calc_len(s: &String) -> usize {// &String : String에 대한 참조자
    s.len()
}

fn main() {
    let s1 = String::from("hello");

    calc_len(&s1); // & 연산자 사용, 참조자
}

```

- 소유권이 없기 때문에 읽기는 가능, 임의로 변경 불가
  - 빌린다고 함
  - 변경하고 싶다면? 

가변 참조자
---------
- &연산자에 mut 붙여서 선언
- 제한사항으로 특정한 스코프 내 특정한 데이터 조각에 대한 가변 참조자를 `하나만` 만들 수 있다

```rust
fn calc_len(s: &mut String) -> usize {// &mut String : String에 대한 가변 참조자
    s.len()
}

fn main() {
    let s1 = String::from("hello");

    calc_len(&mut s1); // & 연산자 사용, 참조자
}
```