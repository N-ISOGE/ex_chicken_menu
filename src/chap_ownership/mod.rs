
fn calc_len(s: &String) -> usize {
    // &String : String에 대한 참조자
    s.len()
}

fn main(){
    let s1 = String::from("hello");
    calc_len(&s1); // & 연산자 사용, 참조자
}