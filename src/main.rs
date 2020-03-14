use std::process;
use std::io::{self, Write};

fn main() {
    println!("Please input an integer to generate its polyprimodal .scl file.");
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
    println!("Working...");
    std::process::exit(match output::make_scl(n) {
        Ok(()) => 0,
        Err(e) => {
            eprint!("{}", e);
            1
        }
    });
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
    }   

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::Io(ref err) => write!(f, "IO error: {}", err),
                Error::Parse(ref err) => write!(f, "Parse error: {}", err),
                Error::TooSmall => write!(f, "IO error: One and two are invalid inputs."),
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
    use std::io;
    use crate::error::Error;

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
        } else {
          Ok(())
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
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    
    pub fn make_scl(x: u32) -> std::io::Result<()> {
        let name = format!("Polyprimodality-{}.scl", x);
        let mut scl = OpenOptions::new()
            .append(true)
            .create(true)
            .open(name)?;
        writeln!(scl, "{} polyprimodal scale through 8n", x)?;
        writeln!(scl, "    {}", 7 * x - 1)?;
        let mut n = x + 1;
        while n != 8 * x {
            writeln!(scl, "{}", Ratio::new(n, x))?;
            n += 1;
        };
        println!("Complete!");
        Ok(())
    }
}
