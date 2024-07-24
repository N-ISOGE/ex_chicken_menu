//!
//! 받은 메세지를 다시 되돌려서 보내는 서버
//!
//! 1. 소켓 생성
//!     - 주소 구조체에 정보 입력: 포트
//! 2. 바인드,
//!
// use std::io;
use std::sync::RwLock;
use windows::{core::*, Win32::System::Threading::*};

static COUNTER: RwLock<i32> = RwLock::new(0);

extern "system" fn example_callback(
    _: PTP_CALLBACK_INSTANCE,
    _: *mut std::ffi::c_void,
    _: PTP_WORK,
) {
    let mut counter = COUNTER.write().unwrap();
    *counter += 1;
}

pub fn main() -> Result<()> {
    unsafe {
        let work = CreateThreadpoolWork(Some(example_callback), None, None)?;

        for _ in 0..10 {
            SubmitThreadpoolWork(work);
        }

        WaitForThreadpoolWorkCallbacks(work, false);
        CloseThreadpoolWork(work);

        let counter = COUNTER.read().unwrap();
        println!("counter: {}", *counter);
    }
    Ok(())
    // 게임 초기화

    // 메인 루프 실행

    //

    // let input_ip = io::stdin();
}
