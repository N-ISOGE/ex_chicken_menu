mod echo_svr;
mod mem_owner_practice;
mod winsock_svr;

pub fn main() {
    println!("playground");
    echo_svr::main().expect("echo in error");
    println!("{:?}", winsock_svr::main());
}
