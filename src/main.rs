use std::env::args;

#[derive(Debug)]
enum Action 
{
    Uppercase,
    Lowercase
}

impl TryFrom<&str> for Action
{
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> 
    {
        match value {
            "uppercase" =>Ok(Action::Uppercase),
            "lowercase" =>Ok(Action::Lowercase),
            _=>Err(())
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect::<Vec<String>>().split_off(1);
    let flag = args.get(0).unwrap().as_str().try_into();
    let input = args.get(1).expect("No input");

    if let Ok(action) = flag
    {
        let output = match action {
            Action::Uppercase => input.to_uppercase(),
            Action::Lowercase => input.to_lowercase()
        };
        println!("{}", output);
    }
}
