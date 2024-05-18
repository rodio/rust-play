use std::{env, net::SocketAddr};

mod arguments;
mod connections;

fn main() {
    let args: Vec<_> = env::args().collect();
    let conf = match arguments::parse_args(args) {
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
            match connections::do_listen(socket_addr) {
                Err(e) => {
                    eprintln!("Error while listening: {}", e);
                    std::process::exit(1);
                }
                Ok(s) => println!("{}", s),
            };
        } else {
            match connections::do_connect(socket_addr) {
                Err(e) => {
                    eprintln!("Error while connecting: {}", e);
                    std::process::exit(1);
                }
                _ => (),
            }
        }
    }
}
