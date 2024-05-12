use core::fmt;
use std::{
    env::{self, Args},
    fmt::{Display, Formatter},
    io::Read,
    net::{AddrParseError, IpAddr, SocketAddr, TcpListener, TcpStream},
    num::ParseIntError,
};

fn main() {
    let mut args = env::args();
    let conf = match parse_args(&mut args) {
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

enum ArgsErr {
    PortNotSpecifiedErr,
    PortParsingErr(ParseIntError),
    HostNotSpecifiedErr,
    HostParsingErr(AddrParseError),
}

impl Display for ArgsErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ArgsErr::PortNotSpecifiedErr => {
                write!(f, "port not specified")
            }
            ArgsErr::PortParsingErr(e) => {
                write!(f, "can't parse port: {}", e)
            }
            ArgsErr::HostNotSpecifiedErr => {
                write!(f, "host not speicifed")
            }
            ArgsErr::HostParsingErr(e) => {
                write!(f, "can't parse host: {}", e)
            }
        }
    }
}

#[derive(Debug)]
struct Config {
    port: Option<u16>,
    host: Option<IpAddr>,
    listen: Option<bool>,
}

fn parse_args(args: &mut Args) -> Result<Config, ArgsErr> {
    let mut config = Config {
        port: None,
        host: None,
        listen: None,
    };

    args.next(); // skipping the binary name

    while let Some(a) = args.next() {
        match a.as_ref() {
            "-l" | "--listen" => {
                config.listen = Some(true);
            }
            _ => {
                if config.host.is_none() {
                    let parse_res = a.parse::<IpAddr>();
                    match parse_res {
                        Ok(addr) => config.host = Some(addr),
                        Err(e) => return Err(ArgsErr::HostParsingErr(e)),
                    }
                    continue;
                }

                if config.port.is_none() {
                    let parse_res = a.to_string().parse::<u16>();
                    match parse_res {
                        Ok(n) => config.port = Some(n),
                        Err(e) => return Err(ArgsErr::PortParsingErr(e)),
                    };
                }
            }
        };
    }

    if config.port.is_none() {
        return Err(ArgsErr::PortNotSpecifiedErr);
    }
    if config.host.is_none() {
        return Err(ArgsErr::HostNotSpecifiedErr);
    }

    Ok(config)
}

fn handle_client(mut stream: TcpStream) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = Vec::new();
    let result = stream.read_to_end(&mut buf);
    result?;
    Ok(buf.to_vec())
}
