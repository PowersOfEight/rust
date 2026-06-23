use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    match listener {
        Ok(listener) => listen(listener),
        Err(_err) => todo!(),
    }
}

fn listen(listener: TcpListener) -> () {
    for stream in listener.incoming() {
        match stream {
            Ok(_s) => println!("Connection established!"),
            Err(_err) => todo!(),
        }
    }
}
