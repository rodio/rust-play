use std::{
    env::{self},
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
};

fn main() {
    let mut args = env::args();
    let conf = match ncrs::parse_args(&mut args) {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("error while parsing arguments: {}", e);
            std::process::exit(2);
        }
    };

    dbg!(&conf);

    if let (Some(port), Some(host), Some(_)) = (conf.port, conf.host, conf.listen) {
        let listener = match TcpListener::bind(SocketAddr::new(host, port)) {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("can't bind: {}", e);
                std::process::exit(1);
            }
        };

        for stream in listener.incoming() {
            let v = handle_client(stream.unwrap()).unwrap();
            println!("{:?}", String::from_utf8(v));
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = Vec::new();
    let result = stream.read_to_end(&mut buf);
    result?;
    Ok(buf.to_vec())
}
