# mutable, shadowing

## mutable

- 공간의 값이 바꿀 수 있는 특성
- 러스트에서는 기본적으로 정하지 않으면 imutable로 선언됨
  - mut 를 적어줘야 변경가능한 것으로 선언됨
  - 컴파일러에서 필요하지 않으면 mut를 제거하는 쪽으로 제안함
  - 안정성을 중요시 함

## shadowing

- 같은 이름을 다시 쓰는 것을 shadowing이라고 부름
- imutable 변수에 다른 값을 대입하는 것은 불가능하며 컴파일러가 경고를 줌
- 대신 같은 이름으로 새로운 변수를 선언 및 정의하는 것은 가능함
- 이전의 변수에 접근이 불가능함이 안전하다?
  - 변수가 언제 변경되는지 제어하기가 쉽다
  - imutable 변수라면 다시 선언하기 전까지 값이 바뀌지 않는다.
  - 선언하는 지점만 고려하면 됨. 선언하고 정의한 값이 내가 얻는 결과값임.

```rust
fn main() {
    println!("Hello, world! {}", give_num(2, 9));
    let woow = 4;
    println!("Hello, world! {}", woow);
    {
        let woow = 10;
        println!("Hello, world! {}", woow);
    }
    println!("Hello, world! {}", woow);
}
```
