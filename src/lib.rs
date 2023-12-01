pub mod menu {

    use crate::uinput;
    use std::io::{self, Write};
    use std::process;

    pub fn dialog(poly: bool) -> u32 {
        let nom = match poly {
            true => "polyprimodality",
            false => "primodality",
        };
        println!("Please input an integer to generate its {} .scl file.", nom);
        let num = loop {
            print!(">>> ");
            io::stdout().flush().unwrap();
            let mut raw_input = uinput::get_parse_check();
            if poly == false {
                if let Ok(input_value) = raw_input {
                    raw_input = uinput::check_prime(input_value);
                }
            }
            match raw_input {
                Ok(input) => break input,
                Err(e) => {
                    eprint!("{} ", e);
                    println!("Input (r) to reset or the any key to quit the program");
                    let choice = uinput::get();
                    match choice {
                        Ok(d) => {
                            if d.trim() == "r" {
                                ();
                            } else {
                                process::exit(0);
                            }
                        }
                        Err(e) => {
                            eprint!("{}", e);
                            process::exit(1);
                        }
                    }
                }
            }
        };
        return num;
    }
}

pub mod error {

    use std::fmt;
    use std::io;
    use std::num;
    use thiserror;

    #[derive(Debug, thiserror::Error)]
    pub enum Kind {
        #[error(transparent)]
        IO(io::Error),
        #[error("You can do better. Aim higher!")]
        TooSmall,
        #[error("Curiously, your input is not prime.")]
        NotPrime,
    }

    #[derive(Debug)]
    pub enum Error {
        Input(Kind),
        Parse(num::ParseIntError),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::Input(ref err) => write!(f, "IO error: {}", err),
                Error::Parse(ref err) => write!(f, "Parse error: {}", err),
            }
        }
    }

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Error {
            Error::Input(Kind::IO(err))
        }
    }

    impl From<num::ParseIntError> for Error {
        fn from(err: num::ParseIntError) -> Error {
            Error::Parse(err)
        }
    }
}

pub mod uinput {
    use crate::error::Error;
    use crate::error::Kind;
    use primapalooza::is_prime;
    use std::io;

    pub fn get() -> Result<String, Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    pub fn parse(input: String) -> Result<u32, Error> {
        let output = input.trim().parse::<u32>()?;
        Ok(output)
    }

    pub fn check(input: u32) -> Result<u32, Error> {
        if input <= 2 {
            Err(Error::Input(Kind::TooSmall))
        } else {
            Ok(input)
        }
    }

    pub fn check_prime(input: u32) -> Result<u32, Error> {
        if is_prime(input as usize) {
            Ok(input)
        } else {
            Err(Error::Input(Kind::NotPrime))
        }
    }

    pub fn get_parse_check() -> Result<u32, Error> {
        let input = get()?;
        let output = parse(input)?;
        let output = check(output)?;
        Ok(output)
    }

    pub struct Config {
        pub from: u8,
        pub to: u8,
        pub poly: bool,
    }
}

pub mod output {
    use super::uinput::Config;
    use num_rational::Ratio;
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    pub fn make_scl(n: u32, ini: Config) -> std::io::Result<()> {
        let nom = match ini.poly {
            true => "polyprimodality",
            false => "primodality",
        };
        let name = format!("{}-{}.scl", nom, n);
        let mut scl = OpenOptions::new().append(true).create(true).open(name)?;
        let min = u32::from(ini.from);
        let max = u32::from(ini.to);
        writeln!(scl, "{} {} scale {}n through {}n", n, nom, min, max)?;
        writeln!(scl, "    {}", (((max - min) * n) - 1))?;
        let mut iter = (min * n) + 1;
        while iter != (max * n) {
            writeln!(scl, "{}", Ratio::new(iter, n))?;
            iter += 1;
        }
        println!("Complete!");
        Ok(())
    }
}
