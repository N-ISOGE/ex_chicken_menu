use error_chain::example_generated::Result;

fn calc_len(s: &String) -> usize {
    // &String : String에 대한 참조자
    s.len()
}

pub fn main() -> Result<()> {
    let s1 = String::from("hello");
    calc_len(&s1); // & 연산자 사용, 참조자
    Ok(())
}
