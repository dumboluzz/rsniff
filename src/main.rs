#[macro_use]
extern crate quicli;
extern crate rayon;
use quicli::prelude::*;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "start", short = "s", default_value = "0")]
    start: u16,
    #[structopt(long = "end", short = "e", default_value = "1023")]
    end: u16,
    #[structopt(long = "threads", short = "n", default_value = "200")]
    threads: usize,
    #[structopt(long = "timeout", short = "t", default_value = "2")]
    timeout: u64,
    ip: IpAddr,
}

main!(|args: Cli| {
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();

    let ports: Vec<u16> = (args.start..args.end)
        .into_par_iter()
        .filter_map(|port| {
            TcpStream::connect_timeout(
                &SocketAddr::new(args.ip, port),
                Duration::new(args.timeout, 0),
            ).ok()
        }).filter_map(|conn| conn.peer_addr().ok())
        .map(|addr| addr.port())
        .collect();

    println!("{} open TCP ports", ports.len());
    for port in ports {
        println!("{}", port);
    }
});
