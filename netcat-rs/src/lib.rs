use core::fmt;
use std::{
    fmt::{Display, Formatter},
    net::{AddrParseError, IpAddr},
    num::ParseIntError,
};

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
pub struct Config {
    pub port: Option<u16>,
    pub host: Option<IpAddr>,
    pub listen: Option<bool>,
}

pub fn parse_args(args: Vec<String>) -> Result<Config, ArgsErr> {
    let mut config = Config {
        port: None,
        host: None,
        listen: None,
    };

    for (i, a) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn listen() {
        let args = vec![
            "/path/to/bin".to_string(),
            "--listen".to_string(),
            "127.0.0.1".to_string(),
            "8080".to_string(),
        ];
        let conf = parse_args(args).unwrap();
        let expected = Config {
            listen: Some(true),
            port: Some(8080),
            host: Some(IpAddr::from([127, 0, 0, 1])),
        };
        assert_eq!(conf, expected);
    }
}
