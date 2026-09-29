use std::env;
use std::process;

struct Command {
    name: String
}

impl Command {
    fn build(args: &Vec<String>) -> Result<Command, &'static str> {
        if args.len() < 2 {
            return Err("not enough args");
        }
        let name = args[1].clone();
        Ok(Command { name })
    }
}

fn run(command: Command) {
    println!("{}", command.name);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Command::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    run(command);
}
