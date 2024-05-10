mod command_line;

use clap::Parser;
use command_line::*;

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Ex01 { id } => {
            println!("id: {}", id)
        }
        SubCommand::Ex02 { case } => match case {
            ExCase::Case01 { name } => {
                println!("name: {}", name)
            }
            ExCase::Case02 => {
                println!("case02")
            }
        },
        SubCommand::Ex03 { case: _case } => {
            println!("use ValueEnum trait is useful")
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
