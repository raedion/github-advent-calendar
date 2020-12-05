mod github;
mod tcplistener;

fn main() {
    github::github();
    tcplistener::tcp_listener();
}