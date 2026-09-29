use std::env;
use std::process;

fn today() -> String {
    let result = process::Command::new("date")
        .arg("+%Y%m%d")
        .output()
        .expect("could not get date");
    String::from_utf8(result.stdout).unwrap().trim().to_string()
}

struct RateArgs {
    id: i8,
    rating: i8,
    // progress: i8,
    notes: String,
    end: String
}

impl RateArgs {
    fn build(args: &[String]) -> Result<RateArgs, &'static str> {
        let mut id = None;
        // let mut notes = None;
        let mut rating = None;
        let mut progress = 100;
        let mut end = today();
        for arg in args {
            let (key, val) = arg
                .strip_prefix("--")
                .and_then(|s| s.split_once('='))
                .ok_or("expected --key=value")?;
            match key {
                "id" => id = Some(val.parse().map_err(|_| "invalid id")?),
                "rating" => rating = Some(val.parse().map_err(|_| "invalid rating")?),
                // "progress" => progress = Some(val.parse().map_err(|_| "invalid progress")?),
                // "date" => date = Some(val.to_string()),
                _ => return Err("unknown arg")
            }
        }
        Ok(RateArgs {
            id: id.ok_or("missing --id")?,
            rating: rating.ok_or("missing --rating")?,
            // progress: progress.ok_or("missing --progress")?,
            notes: "TODO".to_string(),
            end: end,
        })
    }
}

enum Subcommand {
    List,
    Rate(RateArgs)
}

impl Subcommand {
    fn build(args: &[String]) -> Result<Subcommand, &'static str> {
        match args[1].as_str() {
            "ls" => Ok(Subcommand::List),
            "rate" => {
                let args = RateArgs::build(&args[2..])?;
                Ok(Subcommand::Rate(args))
            },
            _ => Err("unknown subcommand")
        }
    }
}

fn rate(args: RateArgs) {
    println!("{}", args.id);
    println!("{}", args.rating);
    println!("{}", args.notes);
    println!("{}", args.end);
}

fn run(subcommand: Subcommand) {
    match subcommand {
        Subcommand::List => println!("ls"),
        Subcommand::Rate(args) => rate(args)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let subcommand = Subcommand::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    run(subcommand);
}
