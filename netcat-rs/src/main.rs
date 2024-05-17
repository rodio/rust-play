use std::{
    env,
    io::{self, stdin, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

fn main() {
    let args: Vec<_> = env::args().collect();
    let conf = match netcat_rs::parse_args(args) {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("error while parsing arguments: {}", e);
            std::process::exit(2);
        }
    };

    dbg!(&conf);

    if let (Some(port), Some(host), Some(listen)) = (conf.port, conf.host, conf.listen) {
        let socket_addr = SocketAddr::new(host, port);
        if listen {
            match do_listen(socket_addr) {
                Err(e) => {
                    eprintln!("Error while listening: {}", e);
                    std::process::exit(1);
                }
                _ => (),
            };
        } else {
            match do_connect(socket_addr) {
                Err(e) => {
                    eprintln!("Error while connecting: {}", e);
                    std::process::exit(1);
                }
                _ => (),
            }
        }
    }
}

fn do_listen(socket_addr: SocketAddr) -> io::Result<()> {
    let listener = TcpListener::bind(socket_addr)?;

    // TODO receive only one connection and return io::Error::new(Utf8Error, error)
    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut buf = Vec::new();
        stream.read_to_end(&mut buf)?;

        match String::from_utf8(buf) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("Can't convert to utf8: {}", e),
        };
    }
    Ok(())
}

fn do_connect(socket_addr: SocketAddr) -> io::Result<usize> {
    let mut stream = TcpStream::connect(socket_addr)?;

    let mut buf = Vec::new();
    stdin().lock().read_to_end(&mut buf)?;

    stream.write(buf.as_slice())
}
