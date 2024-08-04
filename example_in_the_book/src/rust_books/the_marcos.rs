#[macro_export]
macro_rules! my_assert_ne {
    ($left:expr,$right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    eprintln!(" ne {} {} !", *left_val, *right_val);
                    // panic!("equal!");

                    // let kind = $crate::panicking::AssertKind::Ne;
                    // $crate::panicking::assert_failed(
                    //     kind,
                    //     &*left_val,
                    //     &*right_val,
                    //     $crate::option::Option::None,
                    // );
                }
            }
        }
    };
    ($left:expr,$right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    eprintln!("ne {} {}",*left_val,*right_val);

                    eprintln!("{}",std::format_args!($($arg)+));

                    // panic!("equal!");

                    // let kind = $crate::panicking::AssertKind::Ne;
                    // $crate::panicking::assert_failed(
                    //     kind,
                    //     &*left_val,
                    //     &*right_val,
                    //     $crate::option::Option::Some(
                    //         $crate::formate_args!($($arg)+)
                    //     ),
                    // );
                }
            }
        }
    };
}

#[allow(dead_code)]
pub fn main() {
    simple_example();
}

fn simple_example() {
    let a = 3;
    let b = 1 + 2;

    assert_eq!(a, b);
    my_assert_ne!(a, b, "추가 정보");
    assert_eq!(a, b, "기본적으로 정수만 써서 계산된 결과 {}, {}", a, b);
}
