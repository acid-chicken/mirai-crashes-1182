#[derive(Debug)]
pub enum Error {
    InputFailed,
    ParseFailed,
    UnknownValue,
}

#[derive(Debug)]
pub enum NonZeroEnum {
    One = 1,
    Two = 2,
    Three = 3,
}

impl NonZeroEnum {
    pub fn try_from_optional(source: u8) -> Result<Option<Self>, Error> {
        match source {
            0 => Ok(None),
            1 => Ok(Some(NonZeroEnum::One)),
            2 => Ok(Some(NonZeroEnum::Two)),
            3 => Ok(Some(NonZeroEnum::Three)),
            _ => Err(Error::UnknownValue),
        }
    }
}

fn main() {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input).or(Err(Error::InputFailed))
        .and_then(|_| input.trim().parse::<u8>().or(Err(Error::ParseFailed)))
        .and_then(NonZeroEnum::try_from_optional) {
        Ok(v) => {
            println!("{:?}", v);
        },
        Err(error) => {
            eprintln!("input failed: {:?}", error);
        },
    }
}
