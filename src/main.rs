use std::str::FromStr;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum Action {
    Uppercase,
    Lowercase,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "uppercase" => Ok(Action::Uppercase),
            "lowercase" => Ok(Action::Lowercase),
            _ => Err(()),
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// action to apply on the string
    action: Action,

    /// input string
    input: String,

    /// reverse input chars
    #[arg(short, long, action)]
    reverse: bool,

    /// shift each char by a certain amount
    #[arg(short, long, default_value_t = 0)]
    cæsar: u8,
}

fn main() {
    let args = Args::parse();
    let output = args.input.clone();
    let mut output = match args.action {
        Action::Uppercase => output.to_uppercase(),
        Action::Lowercase => output.to_lowercase(),
    };
    if args.reverse {
        output = output.chars().rev().collect();
    }
    output = output
        .chars()
        .map(|c| (c as u8 + args.cæsar) as char)
        .collect();
    println!("{}", output);
}
