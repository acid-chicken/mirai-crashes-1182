#![allow(dead_code)]

macro_rules! strip_leading_plus {
    (+ $($rest: tt)*) => {
        $($rest)*
    }
}

macro_rules! as_result_of_nonzero_enum_optional {
    (@step $($current: literal)+, $source: ident | $error: expr, $value: expr) => {
        if $source == strip_leading_plus!($(+ $current)+) {
            Ok(Some($value))
        } else {
            Err($error)
        }
    };
    (@step $($current: literal)+, $source: ident | $error: expr, $value: expr, $($rest: expr),*) => {
        if $source == strip_leading_plus!($(+ $current)+) {
            Ok(Some($value))
        } else {
            as_result_of_nonzero_enum_optional!(@step 1 $($current)+, $source | $error, $($rest),*)
        }
    };
    ($source: ident | $error: expr => $($value: expr),+) => {
        if $source == 0 {
            Ok(None)
        } else {
            as_result_of_nonzero_enum_optional!(@step 1, $source | $error, $($value),+)
        }
    }
}

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
        as_result_of_nonzero_enum_optional!(source | Error::UnknownValue => NonZeroEnum::One, NonZeroEnum::Two, NonZeroEnum::Three)
    }
}

#[derive(Debug)]
pub struct Container {
    pub value: Option<NonZeroEnum>,
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
