use std::{
    io::{self, stdin, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

pub fn do_listen(socket_addr: SocketAddr) -> io::Result<String> {
    let listener = TcpListener::bind(socket_addr)?;

    let (mut stream, _) = listener.accept()?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;

    String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn do_connect(socket_addr: SocketAddr) -> io::Result<usize> {
    let mut stream = TcpStream::connect(socket_addr)?;

    let mut buf = Vec::new();
    stdin().lock().read_to_end(&mut buf)?;

    stream.write(buf.as_slice())
}
