mod echo_svr;
mod winsock_svr;

pub fn main() {
    println!("playground");
    println!("{:?}", winsock_svr::main());
}
