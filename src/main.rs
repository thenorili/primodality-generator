use std::process;
use std::io::{self, Write};

fn main() {
    println!("Please input a prime to generate its primodal .scl file.");
    let n = loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        match input::prime() {
            Ok(input) => break input,
            Err(e) => {
                eprint!("{} ", e);
                println!("Input (r) to reset or the any key to quit the program");
                let choice = input::get();
                match choice {
                    Ok(d) => {
                        if d.trim() == "r" { (); 
                        } else { process::exit(0); }
                    }
                    Err(e) => {
                        eprint!("{}", e);
                        process::exit(1);
                    }
                }
            }
        }
    };
    println!("{}", n);
    process::exit(0);
}

mod error {
    
    use std::fmt;
    use std::num;
    use std::io;

     #[derive(Debug)]
    pub enum Error {
        Io(io::Error),
        Parse(num::ParseIntError),
        TooSmall,
        NotPrime,
    }   

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::Io(ref err) => write!(f, "IO error: {}", err),
                Error::Parse(ref err) => write!(f, "Parse error: {}", err),
                Error::TooSmall => write!(f, "IO error: One and two are invalid inputs."),
                Error::NotPrime => write!(f, "IO error: Your input is not prime."),
            }
        }
    }

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Error {
            Error::Io(err)
        }
    }

    impl From<num::ParseIntError> for Error {
        fn from(err: num::ParseIntError) -> Error {
            Error::Parse(err)
        }
    }
}


mod input {
    use std::usize; 
    use std::io;
    use crate::error::Error;
    use primapalooza::is_prime;

    pub fn get() -> Result<String, Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    fn parse(input: String) -> Result<u32, Error> {
        let output = input.trim().parse::<u32>()?;
        Ok(output)
    }

    fn check(input: u32) -> Result<(), Error> {
        if input <= 2 { 
            Err(Error::TooSmall)
        } else if is_prime(input as usize) {
            Ok(())
        } else {
            Err(Error::NotPrime)
        }
    }

    pub fn prime() -> Result<u32, Error> {
        let input = get()?;
        let output = parse(input)?;
        check(output)?;
        Ok(output)
    }
}


mod output {
    use num_rational::Ratio;
    use std::fs::File;
    use std::io::prelude::*;
    
    pub fn make_array(d: u32) -> array {
        let mut tonic = Ratio::new(d, d);
        let mut scale = [tonic; 7 * d - 1];
        let mut n = d + 1;
        for x in scale.iter() {
            let x = Ratio::new(n, d);
            let n = n + 1;
        };





























