use core::fmt;
use std::{
    env::Args,
    fmt::{Display, Formatter},
    net::{AddrParseError, IpAddr},
    num::ParseIntError,
};

pub enum ArgsErr {
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
pub struct Config {
    pub port: Option<u16>,
    pub host: Option<IpAddr>,
    pub listen: Option<bool>,
}

pub fn parse_args(args: &mut Args) -> Result<Config, ArgsErr> {
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
