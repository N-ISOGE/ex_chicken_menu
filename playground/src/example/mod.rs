mod echo_svr;
mod winsock_svr;

pub fn main() {
    println!("playground");
    echo_svr::main().expect("echo in error");
    println!("{:?}", winsock_svr::main());
}
