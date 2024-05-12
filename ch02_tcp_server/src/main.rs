mod command_line;
mod httpserver;
mod tcpclient;
mod tcpserver;

use clap::Parser;
use command_line::*;
use mylib::setup_simple_tracing;
use tracing::info;

fn main() {
    setup_simple_tracing();

    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Server { port } => tcpserver::run(port),
        SubCommand::Client { port } => tcpclient::run(port),
        SubCommand::Ex02 { case } => match case {
            ExCase::Case01 { name } => {
                info!("name: {}", name)
            }
            ExCase::Case02 => {
                info!("case02")
            }
        },
        SubCommand::Ex03 { case: _case } => {
            info!("use ValueEnum trait is useful")
        }
    }
}

#[cfg(test)]
mod tests {
    use mylib::add;

    #[test]
    pub fn case01() {
        assert_eq!(2, add(1, 1))
    }
}
