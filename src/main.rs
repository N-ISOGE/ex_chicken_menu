fn main() {
    println!("hello!");
    let s1 = String::from("woow");
    println!("{}", s1);

    take_ownership(s1);
    // println!("{}", s1);

    let x = 4;

    makes_copy(x);

    println!("{}", x);
}

fn take_ownership(input: String) {
    println!("take {}", input);
}

fn makes_copy(input: i32) {
    println!("copy {}", input)
}