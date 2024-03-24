use std::env::args;

enum Flag {
    Reverse,
    Ceasar,
}

#[derive(Debug)]
enum Action {
    Uppercase,
    Lowercase,
}

impl<'a> TryFrom<&'a str> for Flag {
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "-r" | "-R" => Ok(Flag::Reverse),
            "-c" | "-C" => Ok(Flag::Ceasar),
            v => Err(v),
        }
    }
}

impl<'a> TryFrom<&'a str> for Action {
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "uppercase" => Ok(Action::Uppercase),
            "lowercase" => Ok(Action::Lowercase),
            v => Err(v),
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect::<Vec<String>>().split_off(1);
    let action = args.get(0).unwrap().as_str().try_into();
    let input = args.get(1).expect("Gnarpy has found no input 👽");

    let flag: Option<Flag> = match args.get(2) {
        Some(quelqeuchosde) => quelqeuchosde.as_str().try_into().ok(),
        None => None,
    };

    if let Ok(action) = action {
        let mut output: String = match action {
            Action::Uppercase => input.to_uppercase(),
            Action::Lowercase => input.to_lowercase(),
        };
        if let Some(Flag::Reverse) = flag {
            output = output.chars().rev().collect()
        }
        println!("{}", output);
    }
}
