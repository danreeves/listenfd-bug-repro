extern crate listenfd;
use listenfd::ListenFd;

fn main() {
    let mut listenfd = ListenFd::from_env();

    if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        println!("{:?}", listener);
    } else {
        println!("no listener found");
    };
}
